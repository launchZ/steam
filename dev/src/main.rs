fn main() {
    steam::a2s::query_info("142.4.217.38:27016").unwrap();
    steam::a2s::query_players("142.4.217.38:27016").unwrap();
}
