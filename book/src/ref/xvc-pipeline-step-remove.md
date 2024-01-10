# xvc pipeline step remove

## Purpose

Remove a step and all its dependencies and outputs from the pipeline.

## Synopsis

```console
$ xvc pipeline step remove --help
Remove a step from a pipeline

Usage: xvc pipeline step remove --step-name <STEP_NAME>

Options:
  -s, --step-name <STEP_NAME>  Name of the step to remove
  -h, --help                   Print help

```

## Examples

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Let's create a few steps and make them depend on each other.
```console
$ xvc pipeline step new --step-name hello --command 'echo hello >> hello.txt'

$ xvc pipeline step new --step-name world --command 'echo world >> world.txt'

$ xvc pipeline step new --step-name from --command 'echo from >> from.txt'

$ xvc pipeline step new --step-name xvc --command 'echo xvc >> xvc.txt'

```
Let's specify the outputs as well.
```console
$ xvc pipeline step output --step-name hello --output-file hello.txt

$ xvc pipeline step output --step-name world --output-file world.txt

$ xvc pipeline step output --step-name from --output-file from.txt

$ xvc pipeline step output --step-name xvc --output-file xvc.txt

```

Now we can add dependencies between them.
```console
$ xvc pipeline step dependency --step-name xvc --step from

$ xvc pipeline step dependency --step-name from --file world.txt

$ xvc pipeline step dependency --step-name world --step hello

```


Now the pipeline looks like this:
```console
$ xvc pipeline step list
hello: echo hello >> hello.txt (by_dependencies)
world: echo world >> world.txt (by_dependencies)
from: echo from >> from.txt (by_dependencies)
xvc: echo xvc >> xvc.txt (by_dependencies)

$ xvc pipeline dag --format mermaid
flowchart TD
    n0["hello"]
    n1["hello.txt"] --> n0
    n2["world"]
    n0["hello"] --> n2
    n3["world.txt"] --> n2
    n4["from"]
    n3["world.txt"] --> n4
    n5["from.txt"] --> n4
    n6["xvc"]
    n4["from"] --> n6
    n7["xvc.txt"] --> n6


```

When we remove a step, all its dependencies and outputs are removed as well.
```console
$ xvc pipeline step remove --step-name from

```

```console
$ xvc pipeline step list
hello: echo hello >> hello.txt (by_dependencies)
world: echo world >> world.txt (by_dependencies)
xvc: echo xvc >> xvc.txt (by_dependencies)

$ xvc -vvv pipeline dag --format mermaid
[DEBUG][logging/src/lib.rs::237] Terminal logger enabled with level: Error
[DEBUG][logging/src/lib.rs::240] File logger enabled with level: Trace to "/var/folders/tk/3vn311ps4kqdhgykj3jg_p8r0000gn/T//xvc.log"
[TRACE][core/src/types/xvcroot.rs::247] "."
[DEBUG][core/src/types/xvcroot.rs::253] XVC DIR: "[CWD]"
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][ecs/src/ecs/mod.rs::229] dir: "[CWD]/.xvc/ec"
[TRACE][ecs/src/ecs/mod.rs::239] files: [
    "[CWD]/.xvc/ec/1704908070484353",
    "[CWD]/.xvc/ec/1704908070489059",
    "[CWD]/.xvc/ec/1704908070556672",
    "[CWD]/.xvc/ec/1704908070632051",
    "[CWD]/.xvc/ec/1704908070779695",
    "[CWD]/.xvc/ec/1704908070858423",
    "[CWD]/.xvc/ec/1704908070934624",
    "[CWD]/.xvc/ec/1704908071053088",
    "[CWD]/.xvc/ec/1704908071226746",
    "[CWD]/.xvc/ec/1704908071320947",
    "[CWD]/.xvc/ec/1704908071422626",
    "[CWD]/.xvc/ec/1704908071505427",
    "[CWD]/.xvc/ec/1704908071595425",
]
[TRACE][pipeline/src/pipeline/api/dag.rs::65] pipeline_steps: HStore {
    map: {
        XvcEntity(
            3,
            17709525648495250000,
        ): XvcStep {
            name: "world",
        },
        XvcEntity(
            5,
            11911495236914863175,
        ): XvcStep {
            name: "xvc",
        },
        XvcEntity(
            2,
            7818672908942248127,
        ): XvcStep {
            name: "hello",
        },
    },
}
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.14/src/lib.rs::453] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][walker/src/lib.rs::489] ignore_fn: ".xvcignore"
[TRACE][walker/src/lib.rs::491] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::499] entry.path(): "[CWD]/.xvc"
[TRACE][walker/src/lib.rs::499] entry.path(): "[CWD]/.git"
[TRACE][walker/src/lib.rs::504] ignore_path: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::598] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::599] ignore_path: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::607] &content: "
# Add patterns of files xvc should ignore, which could improve
# the performance.
# It's in the same format as .gitignore files.

.DS_Store
"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.14/src/lib.rs::453] built glob set; 0 literals, 3 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][walker/src/lib.rs::749] is_abs: true
[TRACE][walker/src/lib.rs::753] path_str: "[CWD]/.xvc"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "/.xvc"
[TRACE][walker/src/lib.rs::749] is_abs: true
[TRACE][walker/src/lib.rs::753] path_str: "[CWD]/.git"
[TRACE][walker/src/lib.rs::755] final_slash: false
[TRACE][walker/src/lib.rs::777] path: "/.git"
[TRACE][core/src/util/pmp.rs::39] ignore_rules: IgnoreRules {
    root: "[CWD]",
    ignore_patterns: RwLock {
        data: [
            Pattern {
                pattern: Glob {
                    glob: "**/.xvc",
                    re: "(?-u)^(?:/?|.*/)//.xvc$",
                    opts: GlobOptions {
                        case_insensitive: false,
                        literal_separator: false,
                        backslash_escape: true,
                        empty_alternates: false,
                    },
                    tokens: Tokens(
                        [
                            RecursivePrefix,
                            Literal(
                                '.',
                            ),
                            Literal(
                                'x',
                            ),
                            Literal(
                                'v',
                            ),
                            Literal(
                                'c',
                            ),
                        ],
                    ),
                },
                original: ".xvc",
                source: Global,
                effect: Ignore,
                relativity: Anywhere,
                path_kind: Any,
            },
            Pattern {
                pattern: Glob {
                    glob: "**/.git",
                    re: "(?-u)^(?:/?|.*/)//.git$",
                    opts: GlobOptions {
                        case_insensitive: false,
                        literal_separator: false,
                        backslash_escape: true,
                        empty_alternates: false,
                    },
                    tokens: Tokens(
                        [
                            RecursivePrefix,
                            Literal(
                                '.',
                            ),
                            Literal(
                                'g',
                            ),
                            Literal(
                                'i',
                            ),
                            Literal(
                                't',
                            ),
                        ],
                    ),
                },
                original: ".git",
                source: Global,
                effect: Ignore,
                relativity: Anywhere,
                path_kind: Any,
            },
            Pattern {
                pattern: Glob {
                    glob: "**/.DS_Store",
                    re: "(?-u)^(?:/?|.*/)//.DS_Store$",
                    opts: GlobOptions {
                        case_insensitive: false,
                        literal_separator: false,
                        backslash_escape: true,
                        empty_alternates: false,
                    },
                    tokens: Tokens(
                        [
                            RecursivePrefix,
                            Literal(
                                '.',
                            ),
                            Literal(
                                'D',
                            ),
                            Literal(
                                'S',
                            ),
                            Literal(
                                '_',
                            ),
                            Literal(
                                'S',
                            ),
                            Literal(
                                't',
                            ),
                            Literal(
                                'o',
                            ),
                            Literal(
                                'r',
                            ),
                            Literal(
                                'e',
                            ),
                        ],
                    ),
                },
                original: ".DS_Store",
                source: File {
                    path: ".xvcignore",
                    line: 6,
                },
                effect: Ignore,
                relativity: Anywhere,
                path_kind: Any,
            },
        ],
        poisoned: false,
        ..
    },
    whitelist_patterns: RwLock {
        data: [],
        poisoned: false,
        ..
    },
    whitelist_set: RwLock {
        data: GlobSet {
            len: 0,
            strats: [],
        },
        poisoned: false,
        ..
    },
    ignore_set: RwLock {
        data: GlobSet {
            len: 3,
            strats: [
                Extension(
                    ExtensionStrategy(
                        {},
                    ),
                ),
                BasenameLiteral(
                    BasenameLiteralStrategy(
                        {
                            [
                                46,
                                120,
                                118,
                                99,
                            ]: [
                                0,
                            ],
                            [
                                46,
                                68,
                                83,
                                95,
                                83,
                                116,
                                111,
                                114,
                                101,
                            ]: [
                                2,
                            ],
                            [
                                46,
                                103,
                                105,
                                116,
                            ]: [
                                1,
                            ],
                        },
                    ),
                ),
                Literal(
                    LiteralStrategy(
                        {},
                    ),
                ),
                Suffix(
                    SuffixStrategy {
                        matcher: AhoCorasick(
                            dfa::DFA(
                            D 000000: /x00 => 0
                            F 000001:
                             >000002: /x00 => 2
                              000003: /x00 => 0
                            match kind: Standard
                            prefilter: false
                            state length: 4
                            pattern length: 0
                            shortest pattern length: 18446744073709551615
                            longest pattern length: 0
                            alphabet length: 1
                            stride: 1
                            byte classes: ByteClasses(0 => [0-255])
                            memory usage: 16
                            )
                            ,
                        ),
                        map: [],
                        longest: 0,
                    },
                ),
                Prefix(
                    PrefixStrategy {
                        matcher: AhoCorasick(
                            dfa::DFA(
                            D 000000: /x00 => 0
                            F 000001:
                             >000002: /x00 => 2
                              000003: /x00 => 0
                            match kind: Standard
                            prefilter: false
                            state length: 4
                            pattern length: 0
                            shortest pattern length: 18446744073709551615
                            longest pattern length: 0
                            alphabet length: 1
                            stride: 1
                            byte classes: ByteClasses(0 => [0-255])
                            memory usage: 16
                            )
                            ,
                        ),
                        map: [],
                        longest: 0,
                    },
                ),
                RequiredExtension(
                    RequiredExtensionStrategy(
                        {},
                    ),
                ),
                Regex(
                    RegexSetStrategy {
                        matcher: Regex {
                            imp: RegexI {
                                strat: Core {
                                    info: RegexInfo(
                                        RegexInfoI {
                                            config: Config {
                                                match_kind: Some(
                                                    All,
                                                ),
                                                utf8_empty: Some(
                                                    false,
                                                ),
                                                autopre: None,
                                                pre: None,
                                                which_captures: None,
                                                nfa_size_limit: Some(
                                                    Some(
                                                        10485760,
                                                    ),
                                                ),
                                                onepass_size_limit: None,
                                                hybrid_cache_capacity: Some(
                                                    10485760,
                                                ),
                                                hybrid: None,
                                                dfa: None,
                                                dfa_size_limit: None,
                                                dfa_state_limit: None,
                                                onepass: None,
                                                backtrack: None,
                                                byte_classes: None,
                                                line_terminator: None,
                                            },
                                            props: [],
                                            props_union: Properties(
                                                PropertiesI {
                                                    minimum_len: None,
                                                    maximum_len: None,
                                                    look_set: ∅,
                                                    look_set_prefix: ∅,
                                                    look_set_suffix: ∅,
                                                    look_set_prefix_any: ∅,
                                                    look_set_suffix_any: ∅,
                                                    utf8: true,
                                                    explicit_captures_len: 0,
                                                    static_explicit_captures_len: None,
                                                    literal: false,
                                                    alternation_literal: true,
                                                },
                                            ),
                                        },
                                    ),
                                    pre: None,
                                    nfa: thompson::NFA(
                                    ^000000: FAIL
                                    
                                    transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                    )
                                    ,
                                    nfarev: Some(
                                        thompson::NFA(
                                        ^000000: FAIL
                                        
                                        transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                        )
                                        ,
                                    ),
                                    pikevm: PikeVM(
                                        PikeVMEngine(
                                            PikeVM {
                                                config: Config {
                                                    match_kind: Some(
                                                        All,
                                                    ),
                                                    pre: Some(
                                                        None,
                                                    ),
                                                },
                                                nfa: thompson::NFA(
                                                ^000000: FAIL
                                                
                                                transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                )
                                                ,
                                            },
                                        ),
                                    ),
                                    backtrack: BoundedBacktracker(
                                        None,
                                    ),
                                    onepass: OnePass(
                                        None,
                                    ),
                                    hybrid: Hybrid(
                                        Some(
                                            HybridEngine(
                                                Regex {
                                                    forward: DFA {
                                                        config: Config {
                                                            match_kind: Some(
                                                                All,
                                                            ),
                                                            pre: Some(
                                                                None,
                                                            ),
                                                            starts_for_each_pattern: Some(
                                                                true,
                                                            ),
                                                            byte_classes: Some(
                                                                true,
                                                            ),
                                                            unicode_word_boundary: Some(
                                                                true,
                                                            ),
                                                            quitset: None,
                                                            specialize_start_states: Some(
                                                                false,
                                                            ),
                                                            cache_capacity: Some(
                                                                10485760,
                                                            ),
                                                            skip_cache_capacity_check: Some(
                                                                false,
                                                            ),
                                                            minimum_cache_clear_count: Some(
                                                                Some(
                                                                    3,
                                                                ),
                                                            ),
                                                            minimum_bytes_per_state: Some(
                                                                Some(
                                                                    10,
                                                                ),
                                                            ),
                                                        },
                                                        nfa: thompson::NFA(
                                                        ^000000: FAIL
                                                        
                                                        transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                        )
                                                        ,
                                                        stride2: 1,
                                                        start_map: StartByteMap{/x00 => NonWordByte, /x01 => NonWordByte, /x02 => NonWordByte, /x03 => NonWordByte, /x04 => NonWordByte, /x05 => NonWordByte, /x06 => NonWordByte, /x07 => NonWordByte, /x08 => NonWordByte, /t => NonWordByte, 
 => LineLF, /x0B => NonWordByte, /x0C => NonWordByte, /r => LineCR, /x0E => NonWordByte, /x0F => NonWordByte, /x10 => NonWordByte, /x11 => NonWordByte, /x12 => NonWordByte, /x13 => NonWordByte, /x14 => NonWordByte, /x15 => NonWordByte, /x16 => NonWordByte, /x17 => NonWordByte, /x18 => NonWordByte, /x19 => NonWordByte, /x1A => NonWordByte, /x1B => NonWordByte, /x1C => NonWordByte, /x1D => NonWordByte, /x1E => NonWordByte, /x1F => NonWordByte, ' ' => NonWordByte, ! => NonWordByte, /" => NonWordByte, # => NonWordByte, $ => NonWordByte, % => NonWordByte, & => NonWordByte, /' => NonWordByte, ( => NonWordByte, ) => NonWordByte, * => NonWordByte, + => NonWordByte, , => NonWordByte, - => NonWordByte, . => NonWordByte, / => NonWordByte, 0 => WordByte, 1 => WordByte, 2 => WordByte, 3 => WordByte, 4 => WordByte, 5 => WordByte, 6 => WordByte, 7 => WordByte, 8 => WordByte, 9 => WordByte, : => NonWordByte, ; => NonWordByte, < => NonWordByte, = => NonWordByte, > => NonWordByte, ? => NonWordByte, @ => NonWordByte, A => WordByte, B => WordByte, C => WordByte, D => WordByte, E => WordByte, F => WordByte, G => WordByte, H => WordByte, I => WordByte, J => WordByte, K => WordByte, L => WordByte, M => WordByte, N => WordByte, O => WordByte, P => WordByte, Q => WordByte, R => WordByte, S => WordByte, T => WordByte, U => WordByte, V => WordByte, W => WordByte, X => WordByte, Y => WordByte, Z => WordByte, [ => NonWordByte, // => NonWordByte, ] => NonWordByte, ^ => NonWordByte, _ => WordByte, ` => NonWordByte, a => WordByte, b => WordByte, c => WordByte, d => WordByte, e => WordByte, f => WordByte, g => WordByte, h => WordByte, i => WordByte, j => WordByte, k => WordByte, l => WordByte, m => WordByte, n => WordByte, o => WordByte, p => WordByte, q => WordByte, r => WordByte, s => WordByte, t => WordByte, u => WordByte, v => WordByte, w => WordByte, x => WordByte, y => WordByte, z => WordByte, { => NonWordByte, | => NonWordByte, } => NonWordByte, ~ => NonWordByte, /x7F => NonWordByte, /x80 => NonWordByte, /x81 => NonWordByte, /x82 => NonWordByte, /x83 => NonWordByte, /x84 => NonWordByte, /x85 => NonWordByte, /x86 => NonWordByte, /x87 => NonWordByte, /x88 => NonWordByte, /x89 => NonWordByte, /x8A => NonWordByte, /x8B => NonWordByte, /x8C => NonWordByte, /x8D => NonWordByte, /x8E => NonWordByte, /x8F => NonWordByte, /x90 => NonWordByte, /x91 => NonWordByte, /x92 => NonWordByte, /x93 => NonWordByte, /x94 => NonWordByte, /x95 => NonWordByte, /x96 => NonWordByte, /x97 => NonWordByte, /x98 => NonWordByte, /x99 => NonWordByte, /x9A => NonWordByte, /x9B => NonWordByte, /x9C => NonWordByte, /x9D => NonWordByte, /x9E => NonWordByte, /x9F => NonWordByte, /xA0 => NonWordByte, /xA1 => NonWordByte, /xA2 => NonWordByte, /xA3 => NonWordByte, /xA4 => NonWordByte, /xA5 => NonWordByte, /xA6 => NonWordByte, /xA7 => NonWordByte, /xA8 => NonWordByte, /xA9 => NonWordByte, /xAA => NonWordByte, /xAB => NonWordByte, /xAC => NonWordByte, /xAD => NonWordByte, /xAE => NonWordByte, /xAF => NonWordByte, /xB0 => NonWordByte, /xB1 => NonWordByte, /xB2 => NonWordByte, /xB3 => NonWordByte, /xB4 => NonWordByte, /xB5 => NonWordByte, /xB6 => NonWordByte, /xB7 => NonWordByte, /xB8 => NonWordByte, /xB9 => NonWordByte, /xBA => NonWordByte, /xBB => NonWordByte, /xBC => NonWordByte, /xBD => NonWordByte, /xBE => NonWordByte, /xBF => NonWordByte, /xC0 => NonWordByte, /xC1 => NonWordByte, /xC2 => NonWordByte, /xC3 => NonWordByte, /xC4 => NonWordByte, /xC5 => NonWordByte, /xC6 => NonWordByte, /xC7 => NonWordByte, /xC8 => NonWordByte, /xC9 => NonWordByte, /xCA => NonWordByte, /xCB => NonWordByte, /xCC => NonWordByte, /xCD => NonWordByte, /xCE => NonWordByte, /xCF => NonWordByte, /xD0 => NonWordByte, /xD1 => NonWordByte, /xD2 => NonWordByte, /xD3 => NonWordByte, /xD4 => NonWordByte, /xD5 => NonWordByte, /xD6 => NonWordByte, /xD7 => NonWordByte, /xD8 => NonWordByte, /xD9 => NonWordByte, /xDA => NonWordByte, /xDB => NonWordByte, /xDC => NonWordByte, /xDD => NonWordByte, /xDE => NonWordByte, /xDF => NonWordByte, /xE0 => NonWordByte, /xE1 => NonWordByte, /xE2 => NonWordByte, /xE3 => NonWordByte, /xE4 => NonWordByte, /xE5 => NonWordByte, /xE6 => NonWordByte, /xE7 => NonWordByte, /xE8 => NonWordByte, /xE9 => NonWordByte, /xEA => NonWordByte, /xEB => NonWordByte, /xEC => NonWordByte, /xED => NonWordByte, /xEE => NonWordByte, /xEF => NonWordByte, /xF0 => NonWordByte, /xF1 => NonWordByte, /xF2 => NonWordByte, /xF3 => NonWordByte, /xF4 => NonWordByte, /xF5 => NonWordByte, /xF6 => NonWordByte, /xF7 => NonWordByte, /xF8 => NonWordByte, /xF9 => NonWordByte, /xFA => NonWordByte, /xFB => NonWordByte, /xFC => NonWordByte, /xFD => NonWordByte, /xFE => NonWordByte, /xFF => NonWordByte},
                                                        classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI]),
                                                        quitset: ByteSet {
                                                            bits: {},
                                                        },
                                                        cache_capacity: 10485760,
                                                    },
                                                    reverse: DFA {
                                                        config: Config {
                                                            match_kind: Some(
                                                                All,
                                                            ),
                                                            pre: Some(
                                                                None,
                                                            ),
                                                            starts_for_each_pattern: Some(
                                                                true,
                                                            ),
                                                            byte_classes: Some(
                                                                true,
                                                            ),
                                                            unicode_word_boundary: Some(
                                                                true,
                                                            ),
                                                            quitset: None,
                                                            specialize_start_states: Some(
                                                                false,
                                                            ),
                                                            cache_capacity: Some(
                                                                10485760,
                                                            ),
                                                            skip_cache_capacity_check: Some(
                                                                false,
                                                            ),
                                                            minimum_cache_clear_count: Some(
                                                                Some(
                                                                    3,
                                                                ),
                                                            ),
                                                            minimum_bytes_per_state: Some(
                                                                Some(
                                                                    10,
                                                                ),
                                                            ),
                                                        },
                                                        nfa: thompson::NFA(
                                                        ^000000: FAIL
                                                        
                                                        transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                        )
                                                        ,
                                                        stride2: 1,
                                                        start_map: StartByteMap{/x00 => NonWordByte, /x01 => NonWordByte, /x02 => NonWordByte, /x03 => NonWordByte, /x04 => NonWordByte, /x05 => NonWordByte, /x06 => NonWordByte, /x07 => NonWordByte, /x08 => NonWordByte, /t => NonWordByte, 
 => LineLF, /x0B => NonWordByte, /x0C => NonWordByte, /r => LineCR, /x0E => NonWordByte, /x0F => NonWordByte, /x10 => NonWordByte, /x11 => NonWordByte, /x12 => NonWordByte, /x13 => NonWordByte, /x14 => NonWordByte, /x15 => NonWordByte, /x16 => NonWordByte, /x17 => NonWordByte, /x18 => NonWordByte, /x19 => NonWordByte, /x1A => NonWordByte, /x1B => NonWordByte, /x1C => NonWordByte, /x1D => NonWordByte, /x1E => NonWordByte, /x1F => NonWordByte, ' ' => NonWordByte, ! => NonWordByte, /" => NonWordByte, # => NonWordByte, $ => NonWordByte, % => NonWordByte, & => NonWordByte, /' => NonWordByte, ( => NonWordByte, ) => NonWordByte, * => NonWordByte, + => NonWordByte, , => NonWordByte, - => NonWordByte, . => NonWordByte, / => NonWordByte, 0 => WordByte, 1 => WordByte, 2 => WordByte, 3 => WordByte, 4 => WordByte, 5 => WordByte, 6 => WordByte, 7 => WordByte, 8 => WordByte, 9 => WordByte, : => NonWordByte, ; => NonWordByte, < => NonWordByte, = => NonWordByte, > => NonWordByte, ? => NonWordByte, @ => NonWordByte, A => WordByte, B => WordByte, C => WordByte, D => WordByte, E => WordByte, F => WordByte, G => WordByte, H => WordByte, I => WordByte, J => WordByte, K => WordByte, L => WordByte, M => WordByte, N => WordByte, O => WordByte, P => WordByte, Q => WordByte, R => WordByte, S => WordByte, T => WordByte, U => WordByte, V => WordByte, W => WordByte, X => WordByte, Y => WordByte, Z => WordByte, [ => NonWordByte, // => NonWordByte, ] => NonWordByte, ^ => NonWordByte, _ => WordByte, ` => NonWordByte, a => WordByte, b => WordByte, c => WordByte, d => WordByte, e => WordByte, f => WordByte, g => WordByte, h => WordByte, i => WordByte, j => WordByte, k => WordByte, l => WordByte, m => WordByte, n => WordByte, o => WordByte, p => WordByte, q => WordByte, r => WordByte, s => WordByte, t => WordByte, u => WordByte, v => WordByte, w => WordByte, x => WordByte, y => WordByte, z => WordByte, { => NonWordByte, | => NonWordByte, } => NonWordByte, ~ => NonWordByte, /x7F => NonWordByte, /x80 => NonWordByte, /x81 => NonWordByte, /x82 => NonWordByte, /x83 => NonWordByte, /x84 => NonWordByte, /x85 => NonWordByte, /x86 => NonWordByte, /x87 => NonWordByte, /x88 => NonWordByte, /x89 => NonWordByte, /x8A => NonWordByte, /x8B => NonWordByte, /x8C => NonWordByte, /x8D => NonWordByte, /x8E => NonWordByte, /x8F => NonWordByte, /x90 => NonWordByte, /x91 => NonWordByte, /x92 => NonWordByte, /x93 => NonWordByte, /x94 => NonWordByte, /x95 => NonWordByte, /x96 => NonWordByte, /x97 => NonWordByte, /x98 => NonWordByte, /x99 => NonWordByte, /x9A => NonWordByte, /x9B => NonWordByte, /x9C => NonWordByte, /x9D => NonWordByte, /x9E => NonWordByte, /x9F => NonWordByte, /xA0 => NonWordByte, /xA1 => NonWordByte, /xA2 => NonWordByte, /xA3 => NonWordByte, /xA4 => NonWordByte, /xA5 => NonWordByte, /xA6 => NonWordByte, /xA7 => NonWordByte, /xA8 => NonWordByte, /xA9 => NonWordByte, /xAA => NonWordByte, /xAB => NonWordByte, /xAC => NonWordByte, /xAD => NonWordByte, /xAE => NonWordByte, /xAF => NonWordByte, /xB0 => NonWordByte, /xB1 => NonWordByte, /xB2 => NonWordByte, /xB3 => NonWordByte, /xB4 => NonWordByte, /xB5 => NonWordByte, /xB6 => NonWordByte, /xB7 => NonWordByte, /xB8 => NonWordByte, /xB9 => NonWordByte, /xBA => NonWordByte, /xBB => NonWordByte, /xBC => NonWordByte, /xBD => NonWordByte, /xBE => NonWordByte, /xBF => NonWordByte, /xC0 => NonWordByte, /xC1 => NonWordByte, /xC2 => NonWordByte, /xC3 => NonWordByte, /xC4 => NonWordByte, /xC5 => NonWordByte, /xC6 => NonWordByte, /xC7 => NonWordByte, /xC8 => NonWordByte, /xC9 => NonWordByte, /xCA => NonWordByte, /xCB => NonWordByte, /xCC => NonWordByte, /xCD => NonWordByte, /xCE => NonWordByte, /xCF => NonWordByte, /xD0 => NonWordByte, /xD1 => NonWordByte, /xD2 => NonWordByte, /xD3 => NonWordByte, /xD4 => NonWordByte, /xD5 => NonWordByte, /xD6 => NonWordByte, /xD7 => NonWordByte, /xD8 => NonWordByte, /xD9 => NonWordByte, /xDA => NonWordByte, /xDB => NonWordByte, /xDC => NonWordByte, /xDD => NonWordByte, /xDE => NonWordByte, /xDF => NonWordByte, /xE0 => NonWordByte, /xE1 => NonWordByte, /xE2 => NonWordByte, /xE3 => NonWordByte, /xE4 => NonWordByte, /xE5 => NonWordByte, /xE6 => NonWordByte, /xE7 => NonWordByte, /xE8 => NonWordByte, /xE9 => NonWordByte, /xEA => NonWordByte, /xEB => NonWordByte, /xEC => NonWordByte, /xED => NonWordByte, /xEE => NonWordByte, /xEF => NonWordByte, /xF0 => NonWordByte, /xF1 => NonWordByte, /xF2 => NonWordByte, /xF3 => NonWordByte, /xF4 => NonWordByte, /xF5 => NonWordByte, /xF6 => NonWordByte, /xF7 => NonWordByte, /xF8 => NonWordByte, /xF9 => NonWordByte, /xFA => NonWordByte, /xFB => NonWordByte, /xFC => NonWordByte, /xFD => NonWordByte, /xFE => NonWordByte, /xFF => NonWordByte},
                                                        classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI]),
                                                        quitset: ByteSet {
                                                            bits: {},
                                                        },
                                                        cache_capacity: 10485760,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    dfa: DFA(
                                        None,
                                    ),
                                },
                                info: RegexInfo(
                                    RegexInfoI {
                                        config: Config {
                                            match_kind: Some(
                                                All,
                                            ),
                                            utf8_empty: Some(
                                                false,
                                            ),
                                            autopre: None,
                                            pre: None,
                                            which_captures: None,
                                            nfa_size_limit: Some(
                                                Some(
                                                    10485760,
                                                ),
                                            ),
                                            onepass_size_limit: None,
                                            hybrid_cache_capacity: Some(
                                                10485760,
                                            ),
                                            hybrid: None,
                                            dfa: None,
                                            dfa_size_limit: None,
                                            dfa_state_limit: None,
                                            onepass: None,
                                            backtrack: None,
                                            byte_classes: None,
                                            line_terminator: None,
                                        },
                                        props: [],
                                        props_union: Properties(
                                            PropertiesI {
                                                minimum_len: None,
                                                maximum_len: None,
                                                look_set: ∅,
                                                look_set_prefix: ∅,
                                                look_set_suffix: ∅,
                                                look_set_prefix_any: ∅,
                                                look_set_suffix_any: ∅,
                                                utf8: true,
                                                explicit_captures_len: 0,
                                                static_explicit_captures_len: None,
                                                literal: false,
                                                alternation_literal: true,
                                            },
                                        ),
                                    },
                                ),
                            },
                            pool: Pool(
                                Pool {
                                    stacks: [
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                    ],
                                    owner: 0,
                                    owner_val: UnsafeCell { .. },
                                },
                            ),
                        },
                        map: [],
                        patset: Pool(
                            Pool {
                                stacks: [
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                ],
                                owner: 0,
                                owner_val: UnsafeCell { .. },
                            },
                        ),
                    },
                ),
            ],
        },
        poisoned: false,
        ..
    },
}
[TRACE][walker/src/notify.rs::170] watcher: FsEventWatcher {
    paths: 0x0000600002b48120,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600000e48010,
    runloop: Some(
        (
            0x00006000010440c0,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][core/src/util/pmp.rs::129] background_thread: Mutex {
    data: JoinHandle { .. },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/api/dag.rs::86] pipeline_steps: HStore {
    map: {
        XvcEntity(
            3,
            17709525648495250000,
        ): XvcStep {
            name: "world",
        },
        XvcEntity(
            5,
            11911495236914863175,
        ): XvcStep {
            name: "xvc",
        },
        XvcEntity(
            2,
            7818672908942248127,
        ): XvcStep {
            name: "hello",
        },
    },
}
[TRACE][core/src/util/pmp.rs::57] watcher: FsEventWatcher {
    paths: 0x0000600002b48120,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600000e48010,
    runloop: Some(
        (
            0x00006000010440c0,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][pipeline/src/pipeline/api/dag.rs::104] dependency_graph: {
    XvcEntity(
        3,
        17709525648495250000,
    ): [
        (
            XvcEntity(
                2,
                7818672908942248127,
            ),
            Outgoing,
        ),
    ],
    XvcEntity(
        2,
        7818672908942248127,
    ): [
        (
            XvcEntity(
                3,
                17709525648495250000,
            ),
            Incoming,
        ),
    ],
}
[TRACE][core/src/util/pmp.rs::89] fs_event_index: 0
[TRACE][core/src/util/pmp.rs::91] kill_signal_index: 1
[TRACE][core/src/util/pmp.rs::94] "pmp background updater ticks": "pmp background updater ticks"
flowchart TD
    n0["hello"]
    n1["hello.txt"] --> n0
    n2["world"]
    n0["hello"] --> n2
    n3["world.txt"] --> n2
    n4["xvc"]
    n5["xvc.txt"] --> n4

[TRACE][core/src/util/pmp.rs::183] self.background_thread: Mutex {
    data: JoinHandle { .. },
    poisoned: false,
    ..
}
[TRACE][core/src/util/pmp.rs::188] self.background_thread: Mutex {
    data: JoinHandle { .. },
    poisoned: false,
    ..
}
[TRACE][core/src/util/pmp.rs::97] index: 1
[TRACE][lib/src/cli/mod.rs::385] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/cli/mod.rs::388] &cli_opts.command_string: "/Users/iex/github.com/iesahin/xvc/target/debug/xvc --debug pipeline dag --format mermaid"
[TRACE][lib/src/git.rs::28] args: [
    "-C",
    "[CWD]",
    "diff",
    "--name-only",
    "--cached",
]
[TRACE][lib/src/git.rs::58] git_diff_staged_out: ""
[TRACE][lib/src/git.rs::28] args: [
    "-C",
    "[CWD]",
    "add",
    "--verbose",
    "[CWD]/.xvc",
    "*.gitignore",
    "*.xvcignore",
]
[TRACE][lib/src/git.rs::176] git_add_output: ""

```

