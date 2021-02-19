# rustTemplate

Template to develop plugins for [Tranalyzer2](https://tranalyzer.com/)
in [Rust](https://www.rust-lang.org/) using the
[`t2plugin`](https://tranalyzer.com/rustdoc/t2plugin/) crate.

## Create a new plugin

1. [Download](https://tranalyzer.com/getit) and [install](https://tranalyzer.com/install)
   Tranalyzer2.

2. Create a new Rust plugin

        cd $T2HOME/plugins
        t2plugin --rust -c myPluginName
        cd myPluginName

3. Optional: change the `PLUGINORDER` at the top of `autogen.sh`.

4. Fill the different methods of the
   [`T2Plugin`](https://tranalyzer.com/rustdoc/t2plugin/trait.T2Plugin.html) trait
   implementation in `src/lib.rs`.

## Example plugin

<https://github.com/Tranalyzer/rustExample>
