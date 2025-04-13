# diligent <img src="https://rustfoundation.org/wp-content/uploads/2025/01/Untitled-design-1-150x150.png" height="64" align="right" valign="middle">

**This is a port of the [Diligent Engine](https://github.com/DiligentGraphics/DiligentEngine) to Rust**

This port consists of a wrapper around Diligent Engine that follows the Rust paradigm. It provides a high-level safe interface for all of the engine's features.


> [!WARNING]  
> This crate is in its early stages and everything in it is prone to change. Please keep your expectations low.

## The Rust paradigm

### Compile-time checks
This project aims to treat most of the runtime assert errors earlier at compile-time.

For instance : instead of having immediate and deferred contexts as one type `IDeviceContext` and check if the `Flush` method can be called on it at runtime, this port provides 2 types `ImmediateDeviceContext` and `DeferredDeviceContext` that both have a common interface but implement different traits according to the methods that it can call.

Not only this ensures that the code is safe at compile-time, it also helps with the automatic suggestions for modern IDE's.

### Pattern matching for parameter inputs

Some of the "CreateInfo" structs (i.e. `ShaderCreateInfo`) have fields that can be initialized in different ways depending on their pointer values.

Instead of choosing the shader source depending on which pointer of the struct is not set to null :
```cpp
struct ShaderCreateInfo
{
    ...
    /// If source file path is provided, Source and ByteCode members must be null
    const Char* FilePath DEFAULT_INITIALIZER(nullptr);
    /// If shader source is provided, FilePath and ByteCode members must be null
    const Char* Source DEFAULT_INITIALIZER(nullptr);
    /// If shader byte code is provided, FilePath and Source members must be null
    const void* ByteCode DEFAULT_INITIALIZER(nullptr);

    union
    {
        /// Length of the source code, when Source is not null.
        size_t SourceLength DEFAULT_INITIALIZER(0);
        /// Byte code size (in bytes) must not be zero if ByteCode is not null.
        size_t ByteCodeSize;
    };
    ...
}
```

The rust solution provides an enum that can only be one value at any given time. This value contains all the necessary data with the right types.

```rust
pub enum ShaderSource<'a> {
    FilePath(PathBuf),
    SourceCode(&'a str),
    ByteCode(&'a [u8]),
}
```

This greatly clarifies what parameters need to be set in the struct and ensures their integrity and validity.

## Features

Each of the supported graphics backends is a feature of this crate.
For now, the supported backends are `vulkan`, `opengl`, `d3d11` and `d3d12`.

For now, this port is only supported on Linux and Window. But it will be supported on all other platforms.

## Samples
All of the samples present in the [DiligentSamples repo](https://github.com/DiligentGraphics/DiligentSamples) will be ported (rewritten) in rust as examples in this crate.