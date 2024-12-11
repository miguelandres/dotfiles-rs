(function() {
    var type_impls = Object.fromEntries([["dotfiles_actions",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ActionParser%3C'a%3E-for-CreateDirective%3C'a,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/dotfiles_actions/create/directive.rs.html#99-126\">Source</a><a href=\"#impl-ActionParser%3C'a%3E-for-CreateDirective%3C'a,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, F: FileSystem + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>&gt; <a class=\"trait\" href=\"dotfiles_core/action/trait.ActionParser.html\" title=\"trait dotfiles_core::action::ActionParser\">ActionParser</a>&lt;'a&gt; for <a class=\"struct\" href=\"dotfiles_actions/create/directive/struct.CreateDirective.html\" title=\"struct dotfiles_actions::create::directive::CreateDirective\">CreateDirective</a>&lt;'a, F&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.ActionType\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/dotfiles_actions/create/directive.rs.html#100\">Source</a><a href=\"#associatedtype.ActionType\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"dotfiles_core/action/trait.ActionParser.html#associatedtype.ActionType\" class=\"associatedtype\">ActionType</a> = <a class=\"struct\" href=\"dotfiles_actions/create/action/struct.CreateAction.html\" title=\"struct dotfiles_actions::create::action::CreateAction\">CreateAction</a>&lt;'a, F&gt;</h4></section></summary><div class='docblock'>The action type this object parses</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.parse_action\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dotfiles_actions/create/directive.rs.html#102-125\">Source</a><a href=\"#method.parse_action\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dotfiles_core/action/trait.ActionParser.html#tymethod.parse_action\" class=\"fn\">parse_action</a>(\n    &amp;'a self,\n    settings: &amp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"dotfiles_core/settings/enum.Setting.html\" title=\"enum dotfiles_core::settings::Setting\">Setting</a>&gt;,\n    yaml: &amp;<a class=\"enum\" href=\"https://docs.rs/strict-yaml-rust/0.1.0/strict_yaml_rust/strict_yaml/enum.StrictYaml.html\" title=\"enum strict_yaml_rust::strict_yaml::StrictYaml\">StrictYaml</a>,\n    current_dir: &amp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"dotfiles_actions/create/action/struct.CreateAction.html\" title=\"struct dotfiles_actions::create::action::CreateAction\">CreateAction</a>&lt;'a, F&gt;, <a class=\"struct\" href=\"dotfiles_core/error/struct.DotfilesError.html\" title=\"struct dotfiles_core::error::DotfilesError\">DotfilesError</a>&gt;</h4></section></summary><div class='docblock'>Builds a single action of type <a href=\"dotfiles_core/action/trait.ActionParser.html#associatedtype.ActionType\" title=\"associated type dotfiles_core::action::ActionParser::ActionType\">ActionParser::ActionType</a> from StrictYaml tree object\nthat represents the action’s configuration and a default settings object. <a href=\"dotfiles_core/action/trait.ActionParser.html#tymethod.parse_action\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.parse_action_list\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dotfiles_core/action.rs.html#135-140\">Source</a><a href=\"#method.parse_action_list\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dotfiles_core/action/trait.ActionParser.html#method.parse_action_list\" class=\"fn\">parse_action_list</a>(\n    &amp;'a self,\n    settings: &amp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"dotfiles_core/settings/enum.Setting.html\" title=\"enum dotfiles_core::settings::Setting\">Setting</a>&gt;,\n    yaml: &amp;<a class=\"enum\" href=\"https://docs.rs/strict-yaml-rust/0.1.0/strict_yaml_rust/strict_yaml/enum.StrictYaml.html\" title=\"enum strict_yaml_rust::strict_yaml::StrictYaml\">StrictYaml</a>,\n    current_dir: &amp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;Self::<a class=\"associatedtype\" href=\"dotfiles_core/action/trait.ActionParser.html#associatedtype.ActionType\" title=\"type dotfiles_core::action::ActionParser::ActionType\">ActionType</a>&gt;, <a class=\"struct\" href=\"dotfiles_core/error/struct.DotfilesError.html\" title=\"struct dotfiles_core::error::DotfilesError\">DotfilesError</a>&gt;</h4></section></summary><div class='docblock'>Builds a list of actions of type <a href=\"dotfiles_core/action/trait.ActionParser.html#associatedtype.ActionType\" title=\"associated type dotfiles_core::action::ActionParser::ActionType\">ActionParser::ActionType</a> from StrictYaml tree object\nthat represents the actions’ configurations and a default settings object. <a href=\"dotfiles_core/action/trait.ActionParser.html#method.parse_action_list\">Read more</a></div></details></div></details>","ActionParser<'a>","dotfiles_actions::create::directive::NativeCreateDirective","dotfiles_actions::create::directive::FakeCreateDirective"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-CreateDirective%3C'a,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/dotfiles_actions/create/directive.rs.html#68\">Source</a><a href=\"#impl-Clone-for-CreateDirective%3C'a,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + FileSystem + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"dotfiles_actions/create/directive/struct.CreateDirective.html\" title=\"struct dotfiles_actions::create::directive::CreateDirective\">CreateDirective</a>&lt;'a, F&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dotfiles_actions/create/directive.rs.html#68\">Source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"dotfiles_actions/create/directive/struct.CreateDirective.html\" title=\"struct dotfiles_actions::create::directive::CreateDirective\">CreateDirective</a>&lt;'a, F&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">Source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","dotfiles_actions::create::directive::NativeCreateDirective","dotfiles_actions::create::directive::FakeCreateDirective"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-CreateDirective%3C'a,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/dotfiles_actions/create/directive.rs.html#89-97\">Source</a><a href=\"#impl-Default-for-CreateDirective%3C'a,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, F: FileSystem + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"dotfiles_actions/create/directive/struct.CreateDirective.html\" title=\"struct dotfiles_actions::create::directive::CreateDirective\">CreateDirective</a>&lt;'a, F&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dotfiles_actions/create/directive.rs.html#90-96\">Source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; Self</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","dotfiles_actions::create::directive::NativeCreateDirective","dotfiles_actions::create::directive::FakeCreateDirective"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Directive%3C'a%3E-for-CreateDirective%3C'a,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/dotfiles_actions/create/directive.rs.html#68\">Source</a><a href=\"#impl-Directive%3C'a%3E-for-CreateDirective%3C'a,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, F: FileSystem + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>&gt; <a class=\"trait\" href=\"dotfiles_core/directive/trait.Directive.html\" title=\"trait dotfiles_core::directive::Directive\">Directive</a>&lt;'a&gt; for <a class=\"struct\" href=\"dotfiles_actions/create/directive/struct.CreateDirective.html\" title=\"struct dotfiles_actions::create::directive::CreateDirective\">CreateDirective</a>&lt;'a, F&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.get_setting_from_yaml_hash_or_from_context\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dotfiles_core/directive.rs.html#139-144\">Source</a><a href=\"#method.get_setting_from_yaml_hash_or_from_context\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dotfiles_core/directive/trait.Directive.html#method.get_setting_from_yaml_hash_or_from_context\" class=\"fn\">get_setting_from_yaml_hash_or_from_context</a>(\n    &amp;'a self,\n    name: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>,\n    yaml: &amp;<a class=\"enum\" href=\"https://docs.rs/strict-yaml-rust/0.1.0/strict_yaml_rust/strict_yaml/enum.StrictYaml.html\" title=\"enum strict_yaml_rust::strict_yaml::StrictYaml\">StrictYaml</a>,\n    context_settings: &amp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"dotfiles_core/settings/enum.Setting.html\" title=\"enum dotfiles_core::settings::Setting\">Setting</a>&gt;,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"enum\" href=\"dotfiles_core/settings/enum.Setting.html\" title=\"enum dotfiles_core::settings::Setting\">Setting</a>, <a class=\"struct\" href=\"dotfiles_core/error/struct.DotfilesError.html\" title=\"struct dotfiles_core::error::DotfilesError\">DotfilesError</a>&gt;</h4></section></summary><div class='docblock'>Parse a particular setting with its correct type from yaml, fall back to context settings or\ndirective defaults if not found in yaml. Returns error if there is any kind of parsing or\ntyping error</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.get_setting_from_yaml_hash\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dotfiles_core/directive.rs.html#152-156\">Source</a><a href=\"#method.get_setting_from_yaml_hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dotfiles_core/directive/trait.Directive.html#method.get_setting_from_yaml_hash\" class=\"fn\">get_setting_from_yaml_hash</a>(\n    &amp;'a self,\n    name: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>,\n    yaml: &amp;<a class=\"enum\" href=\"https://docs.rs/strict-yaml-rust/0.1.0/strict_yaml_rust/strict_yaml/enum.StrictYaml.html\" title=\"enum strict_yaml_rust::strict_yaml::StrictYaml\">StrictYaml</a>,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"enum\" href=\"dotfiles_core/settings/enum.Setting.html\" title=\"enum dotfiles_core::settings::Setting\">Setting</a>, <a class=\"struct\" href=\"dotfiles_core/error/struct.DotfilesError.html\" title=\"struct dotfiles_core::error::DotfilesError\">DotfilesError</a>&gt;</h4></section></summary><div class='docblock'>Parses an individual setting named <code>name</code> from a yaml hash using the type stored in\n<code>DirectiveData.setting_types</code>.</div></details></div></details>","Directive<'a>","dotfiles_actions::create::directive::NativeCreateDirective","dotfiles_actions::create::directive::FakeCreateDirective"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-FileSystemDirective%3C'a,+F%3E-for-CreateDirective%3C'a,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/dotfiles_actions/create/directive.rs.html#79-87\">Source</a><a href=\"#impl-FileSystemDirective%3C'a,+F%3E-for-CreateDirective%3C'a,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, F: FileSystem + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>&gt; <a class=\"trait\" href=\"dotfiles_actions/filesystem/trait.FileSystemDirective.html\" title=\"trait dotfiles_actions::filesystem::FileSystemDirective\">FileSystemDirective</a>&lt;'a, F&gt; for <a class=\"struct\" href=\"dotfiles_actions/create/directive/struct.CreateDirective.html\" title=\"struct dotfiles_actions::create::directive::CreateDirective\">CreateDirective</a>&lt;'a, F&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fs\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dotfiles_actions/create/directive.rs.html#80-82\">Source</a><a href=\"#method.fs\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dotfiles_actions/filesystem/trait.FileSystemDirective.html#tymethod.fs\" class=\"fn\">fs</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;F</a></h4></section></summary><div class='docblock'>Returns the filesystem instance</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.mut_fs\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dotfiles_actions/create/directive.rs.html#84-86\">Source</a><a href=\"#method.mut_fs\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dotfiles_actions/filesystem/trait.FileSystemDirective.html#tymethod.mut_fs\" class=\"fn\">mut_fs</a>(&amp;mut self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut F</a></h4></section></summary><div class='docblock'>Returns a mutable reference to the filesystem instance</div></details></div></details>","FileSystemDirective<'a, F>","dotfiles_actions::create::directive::NativeCreateDirective","dotfiles_actions::create::directive::FakeCreateDirective"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-HasDirectiveData%3C'a%3E-for-CreateDirective%3C'a,+F%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/dotfiles_actions/create/directive.rs.html#68\">Source</a><a href=\"#impl-HasDirectiveData%3C'a%3E-for-CreateDirective%3C'a,+F%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, F: FileSystem + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>&gt; <a class=\"trait\" href=\"dotfiles_core/directive/trait.HasDirectiveData.html\" title=\"trait dotfiles_core::directive::HasDirectiveData\">HasDirectiveData</a>&lt;'a&gt; for <a class=\"struct\" href=\"dotfiles_actions/create/directive/struct.CreateDirective.html\" title=\"struct dotfiles_actions::create::directive::CreateDirective\">CreateDirective</a>&lt;'a, F&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.directive_data\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dotfiles_actions/create/directive.rs.html#68\">Source</a><a href=\"#method.directive_data\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dotfiles_core/directive/trait.HasDirectiveData.html#tymethod.directive_data\" class=\"fn\">directive_data</a>(&amp;'a self) -&gt; &amp;'a <a class=\"struct\" href=\"dotfiles_core/directive/struct.DirectiveData.html\" title=\"struct dotfiles_core::directive::DirectiveData\">DirectiveData</a></h4></section></summary><div class='docblock'>Returns the directive data for this object</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.name\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dotfiles_core/directive.rs.html#124\">Source</a><a href=\"#method.name\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dotfiles_core/directive/trait.HasDirectiveData.html#method.name\" class=\"fn\">name</a>(&amp;'a self) -&gt; &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a></h4></section></summary><div class='docblock'>Returns the name of the directive.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.defaults\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dotfiles_core/directive.rs.html#129\">Source</a><a href=\"#method.defaults\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dotfiles_core/directive/trait.HasDirectiveData.html#method.defaults\" class=\"fn\">defaults</a>(&amp;'a self) -&gt; &amp;'a <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"dotfiles_core/settings/enum.Setting.html\" title=\"enum dotfiles_core::settings::Setting\">Setting</a>&gt;</h4></section></summary><div class='docblock'>Returns the default settings as configured.</div></details></div></details>","HasDirectiveData<'a>","dotfiles_actions::create::directive::NativeCreateDirective","dotfiles_actions::create::directive::FakeCreateDirective"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[21256]}