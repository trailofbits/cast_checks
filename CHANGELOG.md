# Changelog

## 0.1.6

- Address [dtolnay/proc-macro2#497](https://github.com/dtolnay/proc-macro2/pull/497). This fix allows `cast_checks`'s [`CAST_CHECKS_LOG`](https://github.com/trailofbits/cast_checks?tab=readme-ov-file#cast_checks_log) feature to work with recent nightly compilers. ([7c1144d](https://github.com/trailofbits/cast_checks/commit/7c1144d6cd932eff70ef770b7dcd41a00d26bffd))

## 0.1.5

- Simplify README.md ([#37](https://github.com/trailofbits/cast_checks/pull/37))

## 0.1.4

- Handle boxed trait objects ([#23](https://github.com/trailofbits/cast_checks/pull/23))

## 0.1.3

- Ensure generated expressions parse as block expressions ([cd283d9](https://github.com/trailofbits/cast_checks/commit/cd283d9dc62346cb0538e35de9adfd2185e39772))
- Prevent "unnecessary parentheses" warnings in generated expressions ([44e6bea](https://github.com/trailofbits/cast_checks/commit/44e6bea46ca65bedcaedae22e00df6ad46fdb054))

## 0.1.2

- Improve error and warning messages ([#6](https://github.com/trailofbits/cast_checks/pull/6))

## 0.1.1

- Don't rewrite when the type is inferred ([#5](https://github.com/trailofbits/cast_checks/pull/5))

## 0.1.0

- Initial release
