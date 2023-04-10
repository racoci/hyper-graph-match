import java.lang.ref.WeakReference
import java.util.concurrent.atomic.AtomicBoolean
import kotlin.random.Random
import kotlin.random.Random.Default.nextInt
import kotlin.random.nextUInt

class HashReference<T>(referent: T?) {
    companion object {
        private var idAcc = 0
        private val knownHashes = mutableMapOf<Int, Int>()
    }

    private val reference = WeakReference(referent)
    private val isHashing = AtomicBoolean(false)
    val value: T? get() = reference.get()
    val id = idAcc++
    override fun hashCode(): Int {
        val hashResult = if (isHashing.compareAndSet(false, true)) {
            reference.get()?.hashCode() ?: super.hashCode()
        } else {
            knownHashes[id] ?: super.hashCode().also {
                knownHashes[id] = it
            }
        }
        isHashing.set(false)
        return hashResult

    }

    override fun equals(other: Any?): Boolean {
        if (other is HashReference<*>) {
            return hashCode() == other.hashCode() && value == other.value
        }
        return super.equals(other)
    }
}

class MutableSafeSet<T>(vararg initialValues: T) : MutableSet<T?> {
    private val refs: MutableSet<HashReference<T>> =
        mutableSetOf(*initialValues.map { HashReference(it) }.toTypedArray())

    companion object {
        private val referenced = mutableSetOf<Int>()
    }

    constructor(iter: Iterable<T>) : this() {
        iter.forEach {
            refs.add(HashReference(it))
        }
    }

    override fun add(element: T?): Boolean = refs.add(HashReference(element))
    override fun addAll(elements: Collection<T?>): Boolean = refs.addAll(elements.map { HashReference(it) })
    override val size: Int get() = refs.size
    override fun clear() = refs.clear()
    override fun containsAll(elements: Collection<T?>): Boolean = refs.containsAll(elements.map { HashReference(it) })
    override fun isEmpty(): Boolean = refs.isEmpty()
    override fun iterator(): MutableIterator<T?> = object : MutableIterator<T?> {
        val refsIterator = refs.iterator()
        override fun hasNext(): Boolean = refsIterator.hasNext()
        override fun next(): T? = refsIterator.next().value
        override fun remove() = refsIterator.remove()
    }

    override fun contains(element: T?): Boolean = refs.contains(HashReference(element))
    override fun retainAll(elements: Collection<T?>): Boolean =
        refs.retainAll(elements.map { HashReference(it) }.toSet())

    override fun removeAll(elements: Collection<T?>): Boolean =
        refs.removeAll(elements.map { HashReference(it) }.toSet())

    override fun remove(element: T?): Boolean = refs.remove(HashReference(element))
    override fun toString(): String {
        val string = refs.joinToString(", ", "{", "}") {
            if (it.id !in referenced) {
                referenced.add(it.id)
                it.value.toString()
            } else {
                "<<<${it.id}>>>"
            }
        }
        referenced.clear()
        return string
    }

    override fun hashCode(): Int {
        return if (!isEmpty()) {
            refs.sumOf { it.hashCode() }
        } else {
            super.hashCode()
        }
    }

    fun <R> map(function: (T) -> R): MutableSafeSet<R> {
        return asSequence().filterNotNull().map(function).toSafeSet()
    }
}

fun <T> Sequence<T>.toSafeSet() = MutableSafeSet(asIterable())
fun <T> mutableSafeSetOf(vararg initialValues: T) = MutableSafeSet(*initialValues)
fun <T> safeSet(vararg elements: T): Set<T?> = mutableSafeSetOf(*elements)

interface Hypergraph<V, E> {
    val nodes: Map<V, Set<E>>
    val edges: Map<E, Set<V>>
}

fun <V,E> Hypergraph<V, E>.permute(nodePermutation: List<Int>, edgePermutation: List<Int>): Hypergraph<V, E> {
    val newNodes = mutableMapOf<V, Set<E>>()
    val newEdges = mutableMapOf<E, Set<V>>()
    val nodeList = nodes.keys.toList()
    val nodeToIndex = nodeList.mapIndexed {index, node -> node to index}.associate { it }
    val edgeList = edges.keys.toList()
    val edgeToIndex = edgeList.mapIndexed {index, edge -> edge to index}.associate { it }
    nodePermutation.forEachIndexed { oldIndex, newIndex ->
        val oldNode = nodeList[oldIndex]
        val newNode = nodeList[newIndex]
        newNodes[newNode] = nodes[oldNode]?.mapNotNull { edge ->
            edgeToIndex[edge]?.let { edgeIndex ->
                edgeList[edgePermutation[edgeIndex]]
            }
        }?.toSet() ?: setOf()
    }
    edgePermutation.forEachIndexed { oldIndex, newIndex ->
        val oldEdge = edgeList[oldIndex]
        val newEdge = edgeList[newIndex]
        newEdges[newEdge] = edges[oldEdge]?.mapNotNull { node ->
            nodeToIndex[node]?.let { nodeIndex ->
                nodeList[nodePermutation[nodeIndex]]
            }
        }?.toSet() ?: setOf()
    }

    return object : Hypergraph<V, E> {
        override val nodes: Map<V, Set<E>> = newNodes
        override val edges: Map<E, Set<V>> = newEdges
    }
}

fun <V, E> hypergraphOf(vararg edges: Pair<E, Iterable<V>>): Hypergraph<V, E> {
    val edges = edges.associate { (edge, nodes) ->
        edge to nodes.toSet()
    }

    val nodes: Map<V, Set<E>> = edges.map { (edge: E, nodes: Iterable<V>) ->
        nodes.map {
            it to edge
        }
    }.flatten().groupBy {
        it.first
    }.mapValues {
        it.value.map { (_, edge) ->
            edge
        }.toSet()
    }

    return object : Hypergraph<V, E> {
        override val edges: Map<E, Set<V>> get() = edges
        override val nodes: Map<V, Set<E>> get() = nodes
    }
}



val <V, E> Hypergraph<V, E>.canon: Pair<Map<V, Int>, Map<E, Int>>
    get() {
    val nodeMap = nodes.keys.associateWith { mutableSafeSetOf<Any>() }
    val edgeMap = edges.keys.associateWith { mutableSafeSetOf<Any>() }

    nodes.forEach { (node, edges) ->
        nodeMap[node]?.addAll(edges.map { edgeMap[it] })
    }

    edges.forEach{ (edge, nodes) ->
        edgeMap[edge]?.addAll(nodes.map { nodeMap[it] })
    }

    return nodeMap.map { (a,b) -> a to b.hashCode() }.toMap() to edgeMap.map { (a,b) -> a to b.hashCode() }.toMap()
}

val <A,B> Pair<A?,B?>?.safe: Pair<A?, B?>
    get() {
        return this?.let { (a, b) ->
            a?.let{
                b?.let{
                    a to b
                } ?: null to null
            } ?: null to null
        }  ?: null to null
    }

infix fun <V1, E1, V2, E2> Hypergraph<V1, E1>.mapTo(other: Hypergraph<V2, E2>): Pair<Map<V1, V2>, Map<E1, E2>>? {
    val (thisNodeMap: Map<V1, Int>?, thisEdgeMap: Map<E1, Int>?) = canon.safe
    if (thisNodeMap?.size != nodes.size || thisEdgeMap?.size != edges.size) {
        println("1")
        return null
    }

    val (otherNodeMap: Map<V2, Int>?, otherEdgeMap: Map<E2, Int>?) = other.canon.safe
    if (otherNodeMap?.size != other.nodes.size || otherEdgeMap?.size != other.edges.size) {
        println("2")
        return null
    }

    val reversedNodeMap: Map<Int, V2> = otherNodeMap.map { (node, edges) ->
        edges.hashCode() to node
    }.toMap()

    val reversedEdgeMap: Map<Int, E2> = otherEdgeMap.map { (edge, nodes) ->
        nodes.hashCode() to edge
    }.toMap()

    val nodeMap: Map<V1, V2> = thisNodeMap.mapNotNull { (node, edges) ->
        reversedNodeMap[edges.hashCode()]?.let{
            node to it
        }
    }.toMap()

    val edgeMap: Map<E1, E2> = thisEdgeMap.mapNotNull { (edge, nodes) ->
        reversedEdgeMap[nodes.hashCode()]?.let{
            edge to it
        }
    }.toMap()

    return nodeMap to edgeMap
}

fun UInt.divide(divisor: UInt): Pair<UInt, UInt>? {
    if (divisor <= 0u) {
        return null
    }
    var quotient = 0u
    var remainder = this
    var shiftedDivisor = divisor

    while (shiftedDivisor <= remainder) {
        shiftedDivisor = shiftedDivisor shl 1
    }

    while (shiftedDivisor > divisor) {
        shiftedDivisor = shiftedDivisor shr 1
        quotient = quotient shl 1
        if (remainder >= shiftedDivisor) {
            quotient += 1u
            remainder -= shiftedDivisor
        }
    }

    return quotient to remainder
}

interface RangeList<T> {
    operator fun get(index: UInt): T
    val size: UInt
    val first: T
    val last: T
}

//val CharRange.size get() = (last - first + 1).toUInt()
//operator fun CharRange.plus(other: CharRange): RangeList<Char> {
//    val self = this
//    return object: RangeList<Char> {
//        override val size: UInt get() = self.size + other.size
//        override val first: Char get() = self.first
//        override val last: Char get() = other.last
//
//        override fun get(index: UInt): Char = when {
//            index < self.size -> self.first + index.toInt()
//            index < size -> other.first + (index - self.size).toInt()
//            else -> ' '
//        }
//    }
//}
//
//operator fun RangeList<Char>.plus(other: CharRange): RangeList<Char> {
//    val self = this
//    return object: RangeList<Char> {
//        override val size: UInt get() = self.size + other.size
//        override val first: Char get() = self.first
//        override val last: Char get() = other.last
//        override fun get(index: UInt): Char = when {
//            index < self.size -> self.first + index.toInt()
//            index < size -> other.first + (index - self.size).toInt()
//            else -> ' '
//        }
//    }
//}

val UInt.charString: String get() {
    val validCharset: List<Char> =  ('꯰'+1..'힣') + ('A'..'Z') + ('a'..'z')
    val base = validCharset.size.toUInt()
    val stringBuilder = StringBuilder()
    var number = this
    while (number > 0u) {
        number.divide(base)?.let { (quotient, remainder) ->
            val nextChar = validCharset[remainder.toInt()]
            stringBuilder.append(nextChar)
            number = quotient
        }
    }
    return if (stringBuilder.isEmpty()) "0" else stringBuilder.reverse().toString()
}

fun randomHypergraph(numNodes: UInt, numEdges: UInt, degree: UInt? = null): Hypergraph<String, Int> {
    val nodes = Array(numNodes.toInt()) { it.toUInt().charString }.toSet()
    return hypergraphOf(*Array(numEdges.toInt()) {
        it to nodes.shuffled().take(degree?.toInt() ?: nextInt(1, numNodes.toInt() + 1))
    })
}

val <V, E> Hypergraph<V, E>.nodeNumber get() = nodes.size
val <V, E> Hypergraph<V, E>.edgeNumber get() = edges.size
val <V, E> Hypergraph<V, E>.rankNumber get() = edges.values.maxOf { it.size }
val <V, E> Hypergraph<V, E>.isEmpty get() = nodeNumber == 0 && edgeNumber == 0
val <V, E> Hypergraph<V, E>.matrix: String get() = edges.keys.joinToString("\n") { edge ->
    nodes.keys.joinToString("") { node ->
        if (edges[edge]?.contains(node) == true) node.toString() else ""
    }
}
operator fun <V, E> Hypergraph<V, E>.contains(node: V): Boolean = node in nodes
operator fun <V, E> Hypergraph<V, E>.get(edge: E): Set<V> = edges[edge] ?: setOf()
operator fun <V, E> Hypergraph<V, E>.contains(edge: Set<V>): Boolean = edge.all {
    nodes[it] == edge
}

interface MutableHypergraph<V, E> : Hypergraph<V, E> {
    override val nodes: MutableMap<V, MutableSet<E>>
    override val edges: MutableMap<E, MutableSet<V>>
}

operator fun <V, E> MutableHypergraph<V, E>.plus(node: V) {
    nodes[node] = mutableSetOf()
}

operator fun <V, E> MutableHypergraph<V, E>.set(edge: E, nodes: Iterable<V>) {
    edges[edge] = nodes.toMutableSet()
}

interface Permutable
interface Permutation{
    operator fun times(other: Permutation): Permutation
    operator fun times(other: Permutable): Permutable
}

fun randomPermutation(n: Int): List<Int> {
    val arr = MutableList(n) { it }
    (n - 1 downTo 1).forEach { i ->
        val j = nextInt (i + 1)
        val tmp = arr[i]
        arr[i] = arr[j]
        arr[j] = tmp
    }
    return arr
}

fun testCanonization(minNodes:Int, minEdges: Int, numGraphs: Int, numTests: Int, maxNodes: Int) {
    repeat(numGraphs) { graphNumber ->
        val numNodes = nextInt(minNodes, maxNodes + 1)
        val numEdges = nextInt(minEdges, numNodes + 1)
        val degree = nextInt(minNodes, numNodes + 1)
        val hg = randomHypergraph(numNodes.toUInt(), numEdges.toUInt(), degree.toUInt())
        println("Hyper Graph $graphNumber with $numNodes nodes and $numEdges edges and degree $degree")
        repeat(numTests) { testNumber ->
            val nodePermutation = randomPermutation(hg.nodes.size)
            val edgePermutation = randomPermutation(hg.edges.size)
            val permutedHg = hg.permute(nodePermutation, edgePermutation)
            val (nodeMap, edgeMap) = (hg mapTo permutedHg).safe
            if (!nodeMap.isNullOrEmpty() && !edgeMap.isNullOrEmpty()) {
                println("\t Hyper Graph ${nodeMap.toList().joinToString(" ") { (a,b) ->
                    "$a>$b"
                }}")
            }
        }
    }
}

fun main(args: Array<String>) {
    println("Program arguments: ${args.joinToString()}")
    testCanonization(10, 10, 100, 100000, 100)
}