use collections::*;

fn main() {
    common_thread_safe_collections();
    common_thread_safe_vec();
    common_thread_safe_linkedlist();

    // dashmap
    hashmap_example();

    // flurry
    flurry_hashset();

    // evmap
    evmap_example();

    // scc
    scc_hashmap();
    scc_hashindex();
    // scc_treeindex();
    scc_hashset();
    scc_queue();
}
