pub(crate) fn connect_store(url: Url) -> Result<Store> {
    let store = WnfsStore::new(url, None, DataStoreKind::Memory);
    Ok(store)
}
