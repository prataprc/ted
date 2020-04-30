struct BufferType<T>
where
    T: Plugin + Buffer,
{
    plugin: T,
}
