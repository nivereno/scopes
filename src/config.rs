
//#ifndef SCOPES_CONFIG_H
//#define SCOPES_CONFIG_H

pub const SCOPES_VERSION_MAJOR: usize = 0;
pub const SCOPES_VERSION_MINOR: usize = 19;
pub const SCOPES_VERSION_PATCH: usize = 0;

// trace partial evaluation and code generation
// produces a firehose of information
pub const SCOPES_DEBUG_CODEGEN: usize = 0;

// any location error aborts immediately and can not be caught
pub const SCOPES_EARLY_ABORT: usize = 0;

// print a list of cumulative timers on program exit
pub const SCOPES_PRINT_TIMERS: usize = 0;

// if 0, will never cache modules
pub const SCOPES_ALLOW_CACHE: usize = 1;

// if 1, will warn about missing C type support, such as for some union types
pub const SCOPES_WARN_MISSING_CTYPE_SUPPORT: usize = 0;

// maximum size in bytes of object cache. by default, this is set to 100 MB
pub const SCOPES_MAX_CACHE_SIZE: usize = (100 << 20);
// maximum number of inodes in cache directory
// we keep this one friendly with FAT32, whose limit is 65534
// and some versions of ext, where the limit is 64000
pub const SCOPES_MAX_CACHE_INODES: usize = 63000;

// maximum number of recursions permitted during partial evaluation
// if you think you need more, ask yourself if ad-hoc compiling a pure C function
// that you can then use at compile time isn't the better choice;
// 100% of the time, the answer is yes because the performance is much better.
pub const SCOPES_MAX_RECURSIONS: usize = 64;

// folder name in ~/.cache in which all cache files are stored
pub const SCOPES_CACHE_DIRNAME: &str = "scopes";

// compile native code with debug info if not otherwise specified
pub const SCOPES_COMPILE_WITH_DEBUG_INFO: usize = 1;

//TODO
//#ifndef SCOPES_WIN32;
//#   ifdef _WIN32;
//#   define SCOPES_WIN32;
//#   endif;
//#endif;

//#ifdef SCOPES_WIN32;
//#define SCOPES_USE_WCHAR 1
//#define SCOPES_USE_WCHAR 0;
//#else;
//#define SCOPES_USE_WCHAR 0;
//#endif;

// maximum size of process stack
//#ifdef SCOPES_WIN32;
// on windows, we only get 1 MB of stack
// #define SCOPES_MAX_STACK_SIZE ((1 << 10) * 768)
// but we build with "-Wl,--stack,8388608"
//#define SCOPES_MAX_STACK_SIZE ((1 << 20) * 7);
//#else;
// on linux, the system typically gives us 8 MB
//#define SCOPES_MAX_STACK_SIZE ((1 << 20) * 7);
//#endif;

//#endif // SCOPES_CONFIG_H;