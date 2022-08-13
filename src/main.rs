mod arc_counter;
mod arc_immutable;
mod arc_node_drop;

fn main() {
    arc_counter::count();

    arc_immutable::stack();

    arc_node_drop::node();
}
