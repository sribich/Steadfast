ARENA
POOL

Fixed Size
    - Use tree to manage unallocated??

Lifetimes?
   - Static
   - Level
   - Frame
   - Unknown

Design Goals
    - Low Frag
    - High Utilization

Texture Streaming?
Debugging?


The semantics matter. A lot of game engines use a mark-and-release per-frame allocation buffer. It is temporary throwaway data for that frame's computation. It does not get tracked or freed piecemeal - it gets blown away.

Games are very friendly to that approach- with a bit of thought you can use arenas and object pools to cover 99% of what you need, and cut out all of the failure modes of a general purpose GC or malloc implementation.

1
---
I know this subject quite well and I will later publish a detailed article.

The real run-time cost of memory management done well in a modern game engine written without OOP features is extremely low.

We usually use a few very simple specialized memory allocators, you'd probably be surprised by how simple memory management can be.

The trick is to not use the same allocator when the lifetime is different.

Some resources are allocated once and basically never freed.

Some resources are allocated per level and freed all at once at the end.

Some resources are allocated during a frame and freed all at once when a new frame starts.

And lastly, a few resources are allocated and freed randomly, and here the cost of fragmentation is manageable because we're talking about a few small chunks (like network packets) 
---

2
---

fluffything on Sept 21, 2019 [–]

+1. We have a large Rust code base, and we forbid Vec and the other collections.

Instead, we have different types of global arenas, bump allocators, etc. that you can use. These all pre-allocate memory once at start up, and... that's it.

When you have well defined allocation patterns, allocating a new "object" is just a "last += 1;` and once you are done you deallocate thousands of objects by just doing `last -= size();`.

That's ~0.3 nanoseconds per allocation, and 0.x nano-seconds to "free" a lot of memory.

For comparison, using jemalloc instead puts you at 15-25 ns per allocation and per deallocation, with "spikes" that go up to 200ns depending on size and alignment requirements. So we are talking here a 100-1000x improvement, and very often the improvement is larger because these custom allocators are more predictable, smaller, etc. than a general purpose malloc, so you get better branch prediction, less I-cache misses, etc.
---

3
---
> Do you use any public available crate for those allocators?

Not really, our bump allocator is ~50 LOC, it just allocates a `Box<[u8]>` with a fixed size on initialization, and stores the index of the currently used memory, and that's it.

We then have a `BumpVec<T>` type that uses this allocator (`ptr`, `len`, `cap`). This type has a fixed-capacity, it cannot be moved or cloned, etc. so it ends up being much simpler than `Vec`.
---
