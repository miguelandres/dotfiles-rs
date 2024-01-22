var searchIndex = new Map(JSON.parse('[\
["dotfiles",{"doc":"","t":"H","n":["main"],"q":[[0,"dotfiles"]],"d":[""],"i":[0],"f":"{{}b}","c":[],"p":[[1,"unit"]],"b":[]}],\
["dotfiles_actions",{"doc":"This crate contains all concrete actions and directives. …","t":"CCCCCCCCCCCFNNNNNNNNNNNNNNNFSSNNNNNNNHNNNNNNNNCCFNNNNNNNNNNNNNNNNNNFSSSSSNNNNNNNHNNNNNNNNCCFIINNNNNNNNNNNNNNNNSFSSIINNNNNNNNHNNNNNNNNCCFNNNNNNNNNNNNNNNNSSSSFNNNNNNNHNNNNNNNKMMCFNNNNNNNNNNNNKMMMMNNMCCIFINNNNNNNNNNNNNNNNNNNNSSSISFISSSSNNNNNNNNNHNNNNNNNNNCFNNNNNNNNNNNN","n":["apt","brew","create","exec","filesystem","homebrew_install","install_command","link","ohmyzsh_install","action","directive","AptAction","borrow","borrow_mut","eq","execute","fmt","from","into","new","packages","skip_in_ci","skip_in_ci","try_from","try_into","type_id","vzip","AptDirective","DIRECTIVE_NAME","PACKAGE_SETTING","borrow","borrow_mut","clone","clone_into","default","directive_data","from","init_directive_data","into","parse_action","parse_action_list","to_owned","try_from","try_into","type_id","vzip","action","directive","BrewAction","borrow","borrow_mut","casks","eq","execute","fmt","force_casks","formulae","from","into","new","skip_in_ci","skip_in_ci","taps","try_from","try_into","type_id","vzip","BrewDirective","CASK_SETTING","DIRECTIVE_NAME","FORCE_CASKS_SETTING","FORMULA_SETTING","TAP_SETTING","borrow","borrow_mut","clone","clone_into","default","directive_data","from","init_directive_data","into","parse_action","parse_action_list","to_owned","try_from","try_into","type_id","vzip","action","directive","CreateAction","FakeCreateAction","NativeCreateAction","borrow","borrow_mut","create_parent_dirs","current_dir","directory","eq","execute","fmt","from","into","new","skip_in_ci","try_from","try_into","type_id","vzip","CREATE_PARENT_DIRS_SETTING","CreateDirective","DIRECTIVE_NAME","DIR_SETTING","FakeCreateDirective","NativeCreateDirective","borrow","borrow_mut","clone","clone_into","default","directive_data","from","fs","init_directive_data","into","mut_fs","parse_action","to_owned","try_from","try_into","type_id","vzip","action","directive","ExecAction","borrow","borrow_mut","command","description","echo","eq","execute","fmt","from","into","new","skip_in_ci","try_from","try_into","type_id","vzip","COMMAND_SETTING","DESCRIPTION_SETTING","DIRECTIVE_NAME","ECHO_SETTING","ExecDirective","borrow","borrow_mut","clone","clone_into","default","directive_data","from","init_directive_data","into","parse_action","to_owned","try_from","try_into","type_id","vzip","FileSystemDirective","fs","mut_fs","action","HomebrewInstallAction","borrow","borrow_mut","check_brew_is_installed","default","execute","from","into","new","try_from","try_into","type_id","vzip","InstallCommand","action_description","action_name","args","base_command","execute","formatted_item_list","items","action","directive","FakeLinkAction","LinkAction","NativeLinkAction","borrow","borrow_mut","create_parent_dirs","eq","execute","fmt","force","from","ignore_missing_target","into","new","path","relink","resolve_symlink_target","skip_in_ci","target","try_from","try_into","type_id","vzip","CREATE_PARENT_DIRS_SETTING","DIRECTIVE_NAME","FORCE_SETTING","FakeLinkDirective","IGNORE_MISSING_TARGET_SETTING","LinkDirective","NativeLinkDirective","PATH_SETTING","RELINK_SETTING","RESOLVE_SYMLINK_TARGET_SETTING","TARGET_SETTING","borrow","borrow_mut","clone","clone_into","default","directive_data","from","fs","fs","init_directive_data","into","mut_fs","parse_action","parse_shortened_action","to_owned","try_from","try_into","type_id","vzip","action","OhMyZshInstallAction","borrow","borrow_mut","check_oh_my_zsh_is_installed","check_zsh_is_installed","execute","from","into","new","try_from","try_into","type_id","vzip"],"q":[[0,"dotfiles_actions"],[9,"dotfiles_actions::apt"],[11,"dotfiles_actions::apt::action"],[27,"dotfiles_actions::apt::directive"],[46,"dotfiles_actions::brew"],[48,"dotfiles_actions::brew::action"],[67,"dotfiles_actions::brew::directive"],[89,"dotfiles_actions::create"],[91,"dotfiles_actions::create::action"],[110,"dotfiles_actions::create::directive"],[133,"dotfiles_actions::exec"],[135,"dotfiles_actions::exec::action"],[152,"dotfiles_actions::exec::directive"],[172,"dotfiles_actions::filesystem"],[175,"dotfiles_actions::homebrew_install"],[176,"dotfiles_actions::homebrew_install::action"],[189,"dotfiles_actions::install_command"],[197,"dotfiles_actions::link"],[199,"dotfiles_actions::link::action"],[222,"dotfiles_actions::link::directive"],[252,"dotfiles_actions::ohmyzsh_install"],[253,"dotfiles_actions::ohmyzsh_install::action"],[266,"dotfiles_core::error"],[267,"core::result"],[268,"core::fmt"],[269,"core::fmt"],[270,"alloc::vec"],[271,"core::any"],[272,"dotfiles_core::directive"],[273,"dotfiles_core::settings"],[274,"strict_yaml_rust::strict_yaml"],[275,"std::path"],[276,"filesystem"],[277,"std::path"],[278,"core::default"],[279,"dotfiles_core::settings"],[280,"core::option"],[281,"subprocess::builder::exec"],[282,"core::fmt"]],"d":["This module contains the AptAction and AptDirective","This module contains the BrewAction and BrewDirective","This module contains the CreateAction and CreateDirective …","This module contains the ExecAction and ExecDirective used …","Module that contains interfaces common to directives that …","This module contains the HomebrewInstallAction","Module that contains common code for all commands that …","This module contains the LinkAction and LinkDirective …","This module contains the OhMyZshInstallAction","This module contains the AptAction that installs packages …","This module defines AptDirective.","AptAction Installs software using apt.","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Constructs a new AptAction","List of packages to install.","Skips this action if it is running in a CI environment.","","","","","","A directive that can build AptActions to install packages","Name of the APT directive","The string that identifies the list of packages to install","","","","","","","Returns the argument unchanged.","Initialize the defaults for the AptDirective.","Calls <code>U::from(self)</code>.","","Parse the list of actions from yaml, in this case it’s …","","","","","","This module contains the BrewAction that installs a brew …","This module defines BrewDirective.","BrewAction Installs software using homebrew.","","","List of casks to install. Casks usually are macOS apps …","","","","Passes <code>--force</code> to <code>brew install --cask</code> to prevent the …","List of brew formulae to <code>brew install</code>, usually command …","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Constructs a new BrewAction","Skips this action if it is running in a CI environment.","","List of repositories to tap into using <code>brew tap</code>.","","","","","A directive that can build BrewActions to install …","The string that identifies the list of casks to install","Name of the Brew directive","force casks to deal with previously installed apps","The string that identifies the list of formulae to install","The string that identifies the list of taps to install","","","","","","","Returns the argument unchanged.","Initialize the defaults for the BrewDirective.","Calls <code>U::from(self)</code>.","","Parse the list of actions from yaml, in this case it’s …","","","","","","This module contains the CreateAction that creates a new …","This module defines CreateDirective.","CreateAction creates a new directory when executed","A Fake create action that works on a fake test filesystem.","A native create action that works on the real filesystem.","","","Force creation of the directory and all its parents if …","Current directory that will be used to determine relative …","Directory to create. Can be absolute or relative.","","Creates the <code>directory</code>.","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Constructs a new instance of CreateAction","","","","","","Constant for the name of the <code>create_parent_dirs</code> Setting …","A directive that can build CreateActions to create …","Constant for the name of the <code>create</code> directive.","Constant for the name of the <code>directory</code> argument that …","CreateDirective that uses the native FakeFileSystem for …","CreateDirective that uses the native OsFileSystem.","","","","","","","Returns the argument unchanged.","","Initializes the default configuration for the …","Calls <code>U::from(self)</code>.","","","","","","","","This module contains the ExecAction that executes a …","This module defines ExecDirective which represents …","ExecAction Installs software using homebrew.","","","The command to run","Description for the command to run.","Whether to print out the command for clarity.","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Create a new Exec Action that will run from the parent …","","","","","","Command to run","Optional description for the command to run","Name of the Exec directive","Echo the command to run before running it.","A directive that can build ExecActions to run commands","","","","","","","Returns the argument unchanged.","Initialize the defaults for the BrewDirective.","Calls <code>U::from(self)</code>.","","","","","","","Common trait for all the directives that use a Filesystem","Returns the filesystem instance","Returns a mutable reference to the filesystem instance","This module contains the HomebrewInstallAction that …","HomebrewInstallAction installs homebrew.","","","Returns true if it can find a brew command","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Constructs a new HomebrewInstallAction","","","","","Trait that represents a command that installs an item …","The description of the action to run, i.e. “Installing …","The name of the action to run, i.e. “cask”","The arguments to pass to the command","The base command to run","Runs the command to execut","a list of items to display","The item actually being installed, for example a homebrew …","This module contains the LinkAction that creates a new …","This module defines LinkDirective.","A Fake create action that works on a fake test filesystem.","LinkAction creates a new symlink <code>path</code> that points to <code>target</code>…","A native create action that works on the real filesystem.","","","Create all parent directories if they do not exist already","","","","Force to replace an existing file or directory when …","Returns the argument unchanged.","Succeed even if <code>target</code> doesn’t point to an existing file …","Calls <code>U::from(self)</code>.","Constructs a new LinkAction","Path of the new symlink","Force to re-create the symlink if it exists already","If the target is another symlink, resolve the ultimate …","","Path that the symlink points to.","","","","","Create parent dirs if they don’t exist","Name of the link directive","Force setting, replaces any other file or directory","LinkDirective that uses the native FakeFileSystem for …","Create the symlink even if the target file does not exist","A directive that can build LinkActions to create …","LinkDirective that uses the native OsFileSystem.","Path setting (path of the symlink)","Relink setting, if true the action relinks an existing …","Resolves the target if it is a symlink and uses the final …","Target setting (path to the file the symlink points to)","","","","","","","Returns the argument unchanged.","","Returns the FileSystem instance being used.","Initialize the defaults for the LinkDirective.","Calls <code>U::from(self)</code>.","","","Parse a shortened action with only link name to target name","","","","","","This module contains the OhMyZshInstallAction that sets up …","OhMyZshInstallAction sets up ohmyzsh.","","","Returns true if the $ZSH environment var is set","Returns true if it can find a brew command","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Constructs a new OhMyZshInstallAction","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,11,11,11,11,11,11,11,0,11,11,11,11,11,11,11,11,0,0,0,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,0,0,0,0,0,0,17,17,17,17,17,17,17,0,17,17,17,17,17,17,17,17,0,0,0,0,0,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,18,0,0,0,0,0,0,21,21,21,21,21,21,21,21,0,21,21,21,21,21,21,21,21,0,0,0,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,0,0,0,0,0,29,29,29,29,29,29,29,0,29,29,29,29,29,29,29,0,30,30,0,0,31,31,31,31,31,31,31,31,31,31,31,31,0,32,32,32,32,32,32,32,0,0,0,0,0,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,0,0,0,0,0,0,0,0,0,0,0,37,37,37,37,37,37,37,37,37,0,37,37,37,37,37,37,37,37,37,0,0,38,38,38,38,38,38,38,38,38,38,38,38],"f":"````````````{ce{}{}}0{{bb}d}{b{{j{fh}}}}{{bl}n}{cc{}}4{{d{Ab{A`}}}b}{b{{Ab{A`}}}}{bd}0{c{{j{e}}}{}{}}0{cAd{}}9```99{AfAf}{{ce}f{}{}}{{}Af}{AfAh}9{{}Ah}>{{AfAjAlAn}{{j{bh}}}}{{AfAjAlAn}{{j{{Ab{b}}h}}}}{ce{}{}}9980```00{B`{{Ab{A`}}}}{{B`B`}d}{B`{{j{fh}}}}{{B`l}n}{B`d}4{cc{}}6{{dd{Ab{A`}}{Ab{A`}}{Ab{A`}}}B`}226{c{{j{e}}}{}{}}0{cAd{}}9``````99{BbBb}{{ce}f{}{}}{{}Bb}{BbAh}7{{}Ah}>{{BbAjAlAn}{{j{B`h}}}}{{BbAjAlAn}{{j{{Ab{B`}}h}}}}{ce{}{}}9980`````00{{{Bd{c}}}dBf}{{{Bd{c}}}BhBf}{{{Bd{c}}}A`Bf}{{{Bd{c}}{Bd{c}}}dBf}{{{Bd{c}}}{{j{fh}}}Bf}{{{Bd{c}}l}nBf}{cc{}}7{{cdA`dBh}{{j{{Bd{c}}h}}}Bf}7{c{{j{e}}}{}{}}0{cAd{}}:``````::{{{Bj{c}}}{{Bj{c}}}{BlBfBn}}{{ce}f{}{}}{{}{{Bj{c}}}{BfBn}}{{{Bj{c}}}Ah{BfBn}}7{{{Bj{c}}}c{BfBn}}{{}Ah}{ce{}{}}2{{{Bj{c}}{Cb{A`C`}}AlAn}{{j{{Bd{c}}h}}}{BfBn}}19981```11{CdCf}{Cd{{Ch{A`}}}}{Cdd}{{CdCd}d}{Cd{{j{fh}}}}{{Cdl}n}{cc{}}8{{dA`{Ch{A`}}dAn}{{j{Cdh}}}}5{c{{j{e}}}{}{}}0{cAd{}};`````;;{CjCj}{{ce}f{}{}}{{}Cj}{CjAh}7{{}Ah}{ce{}{}}{{Cj{Cb{A`C`}}AlAn}{{j{Cdh}}}}18871`{ClcBf}0``22{Cnd}{{}Cn}{Cn{{j{fh}}}}>51<<;5`{D`Cf}0{D`{{Ab{A`}}}}{D`Db}{D`{{j{fh}}}}2{D`{{Ab{c}}}Dd}`````::{{{Df{c}}}d{BfDh}}{{{Df{c}}{Df{c}}}d{BfDh}}{{{Df{c}}}{{j{fh}}}{BfDh}}{{{Df{c}}l}n{BfDh}}3{cc{}}4?{{cA`A`AjAjBh}{{j{{Df{c}}h}}}{BfDh}}{{{Df{c}}}A`{BfDh}}6660{c{{j{e}}}{}{}}0{cAd{}}{ce{}{}}```````````00{{{Dj{c}}}{{Dj{c}}}{BlBfDhBn}}{{ce}f{}{}}{{}{{Dj{c}}}{BfDhBn}}{{{Dj{c}}}Ah{BfDhBn}}9{{{Dj{c}}}c{BfDhBn}}0{{}Ah}61{{{Dj{c}}AjAlAn}{{j{{Df{c}}h}}}{BfDhBn}}079987``77{Dld}0{Dl{{j{fh}}}}>9{dDl}<<;:","c":[],"p":[[5,"AptAction",11],[1,"bool"],[1,"unit"],[5,"DotfilesError",266],[6,"Result",267],[5,"Formatter",268],[8,"Result",268],[5,"String",269],[5,"Vec",270],[5,"TypeId",271],[5,"AptDirective",27],[5,"DirectiveData",272],[8,"Settings",273],[6,"StrictYaml",274],[5,"Path",275],[5,"BrewAction",48],[5,"BrewDirective",67],[5,"CreateAction",91],[10,"FileSystem",276],[5,"PathBuf",275],[5,"CreateDirective",110],[10,"Clone",277],[10,"Default",278],[6,"Setting",273],[5,"HashMap",279],[5,"ExecAction",135],[1,"str"],[6,"Option",280],[5,"ExecDirective",152],[10,"FileSystemDirective",172],[5,"HomebrewInstallAction",176],[10,"InstallCommand",189],[5,"Exec",281],[10,"Display",268],[5,"LinkAction",199],[10,"UnixFileSystem",276],[5,"LinkDirective",222],[5,"OhMyZshInstallAction",253]],"b":[[21,"impl-AptAction%3C\'a%3E"],[22,"impl-ConditionalAction%3C\'a%3E-for-AptAction%3C\'a%3E"],[60,"impl-BrewAction%3C\'a%3E"],[61,"impl-ConditionalAction%3C\'a%3E-for-BrewAction%3C\'a%3E"],[240,"impl-FileSystemDirective%3C\'a,+F%3E-for-LinkDirective%3C\'a,+F%3E"],[241,"impl-LinkDirective%3C\'a,+F%3E"]]}],\
["dotfiles_core",{"doc":"The core of Dotfiles-rs is basically a set of directives …","t":"EEEECCCCCCCKKRKSNMHMNMKFKNNNNNNMNNNNNNNNNNNNNNPFGPPPPPPPHNNNNNNHNNNNHNNNNNNNNNNNNNHNNNNNNNNOOOOOOOHHHPPGIPNNNNNNNHNHNNNNHHHHHHHHHHHHHHHHHHHHHHHH","n":["Action","Directive","Setting","Settings","action","directive","error","exec_wrapper","path","settings","yaml_util","Action","ActionParser","ActionType","ConditionalAction","SKIP_IN_CI_SETTING","check_conditions_and_execute","execute","is_running_in_ci","parse_action","parse_action_list","skip_in_ci","Directive","DirectiveData","HasDirectiveData","borrow","borrow_mut","clone","clone_into","defaults","defaults","directive_data","fmt","from","from","get_setting_from_yaml_hash","get_setting_from_yaml_hash_or_from_context","into","name","name","parse_context_defaults","parse_setting_value","to_owned","try_from","try_into","type_id","CoreError","DotfilesError","ErrorType","ExecutionError","FileSystemError","IncompleteConfigurationError","InconsistentConfigurationError","TestingErrorActionSucceedsWhenItShouldFail","UnexpectedYamlTypeError","YamlParseError","add_directive_error_prefix","add_message_prefix","borrow","borrow","borrow_mut","borrow_mut","error_type","execution_error","fmt","fmt","fmt","fmt","fold_until_first_err","from","from","from","from_io_error","from_wrong_yaml","into","into","is_fs_error","is_inconsistent_config","is_missing_config","is_wrong_yaml","is_yaml_parse_error","message","process_until_first_err","to_string","to_string","try_from","try_from","try_into","try_into","type_id","type_id","encountered","exit_status","expected","fs_error","missing_field","popen_error","scan_error","execute_commands","convert_path_to_absolute","process_home_dir_in_path","Boolean","Integer","Setting","Settings","String","borrow","borrow_mut","clone","clone_into","eq","fmt","from","initialize_settings_object","into","parse_setting","to_owned","try_from","try_into","type_id","fold_hash_until_first_err","get_boolean_from_yaml_hash","get_boolean_setting_from_context","get_boolean_setting_from_yaml_or_context","get_integer_from_yaml_hash","get_integer_setting","get_integer_setting_from_yaml_or_context","get_optional_string_array_from_yaml_hash","get_setting_from_context","get_setting_from_yaml_hash","get_string_array_from_yaml_hash","get_string_content_or_keyed_value","get_string_from_yaml_hash","get_string_setting","get_string_setting_from_yaml_or_context","map_yaml_array","parse_as_array","parse_as_boolean","parse_as_integer","parse_as_string","parse_as_string_array","process_value_from_yaml_hash","process_yaml_hash_until_first_err","read_yaml_file"],"q":[[0,"dotfiles_core"],[11,"dotfiles_core::action"],[22,"dotfiles_core::directive"],[46,"dotfiles_core::error"],[91,"dotfiles_core::error::ErrorType"],[98,"dotfiles_core::exec_wrapper"],[99,"dotfiles_core::path"],[101,"dotfiles_core::settings"],[120,"dotfiles_core::yaml_util"],[144,"core::result"],[145,"strict_yaml_rust::strict_yaml"],[146,"std::path"],[147,"alloc::vec"],[148,"core::fmt"],[149,"core::fmt"],[150,"core::any"],[151,"subprocess::popen"],[152,"core::option"],[153,"subprocess::os_common"],[154,"core::iter::traits::collect"],[155,"core::ops::function"],[156,"std::io::error"],[157,"subprocess::builder::exec"],[158,"std::path"]],"d":["","","","","This module contains the base trait for all Actions.","This module contains the base trait for all Directive and …","Module for the error handling classes and enums.","Wraps some logic to run external commands and handle errors","Contains helpful functions to deal with paths in the …","This module contains the definition of a setting and code …","Module that defines helper functions to process YAML …","An action to be run by a the dotfiles runtime.","Trait to parse a specific action type from StrictYaml.","The action type this object parses","Trait for actions to be skippable under certain conditions.","Skip this whole action in CI environments.","Checks that the conditions allow for executing this …","Executes the action.","Whether the execution environment is presumed to be CI","Builds a single action of type ActionParser::ActionType …","Builds a list of actions of type ActionParser::ActionType …","Whether to skip this action in Continuous Integration …","A parser for action steps, each directive represents a …","A struct that contains the default settings for a …","A trait for all directives, it is shared between …","","","","","Returns the default settings as configured.","Default settings for this directive.","Returns the directive data for this object","","Returns the argument unchanged.","Constructs a new directive from a name and a set of …","Parses an individual setting named <code>name</code> from a yaml hash …","Parse a particular setting with its correct type from …","Calls <code>U::from(self)</code>.","Returns the name of the directive.","Unique name of this directive.","Parses all settings for this directive from StrictYaml, …","<code>DirectiveData.setting_types</code>.","","","","","A core logic error for Dotfiles-rs","Struct that represents an error that happened while …","A collection of types of errors that may occur while …","An error occurred while running a command necessary for …","A filesystem error that was encountered while either …","The configuration is missing a required field","The configuration file is inconsistent with itself or with …","An error only for testing, the action that should fail …","Received an StrictYaml object of an unexpected type","An error that occurred while parsing the StrictYaml file","Adds a prefix to an error with the name of the directive …","Adds a prefix to the existing message","","","","","Error type","Creates an ErrorType::ExecutionError","","","","","Executes the <code>process_function</code> on each of the items in the …","Returns the argument unchanged.","Creates a new Dotfiles error with the given message and …","Returns the argument unchanged.","Creates a new Dotfiles error with the given message and …","Creates a new Dotfiles error with the given message and …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns whether the error is a Fs error.","Returns whether the error is an Inconsistent Config.","returns whether the underlying error is a missing …","Returns whether the error is a wrong yaml type.","Returns whether the error is a wrong yaml type.","Human-readable error message","Executes the <code>process_function</code> on each of the items in the …","","","","","","","","","What we got instead of the expected type.","If the command attempted to execute but failed for some …","An example of what we expected.","The underlying filesystem error.","Name of the field missing in the configuration","If the command could not execute for some reason the …","The underlying scan error","Executes the <code>cmd</code> and waits for it to finish.","Converts a file path to absolute if it is relative. If …","Checks for ~ and replaces it with a home directory if …","A boolean value for a setting","An Integer value for a setting","Represents a value for a setting","The Settings object is a hashmap of option names to a …","A string value for a setting","","","","","","","Returns the argument unchanged.","Returns a Settings object from an array as a bit of …","Calls <code>U::from(self)</code>.","Parse a setting from StrictYaml given a particular setting …","","","","","Process each element of the hash with the <code>process_function</code> …","Gets a specific boolean setting from a yaml hash","Gets a boolean value for the setting named <code>name</code>.","Gets a Boolean value from YAML or context.","Gets a specific integer setting from a yaml hash","Gets a Int value for the setting named <code>name</code>.","Gets a Integer value from YAML or context.","Gets a specific string array setting from a yaml hash, but …","Gets a String value for the setting named <code>name</code>.","Gets a specific setting from a yaml hash","Gets a specific string array setting from a yaml hash","Gets the content of this YAML node or the value for a …","Gets a specific string setting from a yaml hash","Gets a String value for the setting named <code>name</code>.","Gets a String value from YAML or context.","Calls a processing function on all elements of an array, …","Parse a yaml element as an array.","Parse a yaml element as boolean.","Parse a yaml element as Integer.","Parse a yaml element as string, will convert booleans and …","Gets a native <code>Vec&lt;String&gt;</code> from a StrictYaml::Array. It …","Gets the value for a specified key in a yaml hash and does …","Executes the <code>process_function</code> on each of the items in the …","Reads a StrictYaml File. Returns Error in case of a syntax …"],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,8,0,0,1,5,0,8,8,1,0,0,0,13,13,13,13,14,13,14,13,13,13,18,18,13,14,13,13,13,13,13,13,13,22,0,0,22,22,22,22,22,22,22,0,3,22,3,22,3,3,0,22,22,3,3,0,22,3,3,3,3,22,3,3,3,3,3,3,3,0,22,3,22,3,22,3,22,3,35,36,35,37,38,36,39,0,0,0,20,20,0,0,20,20,20,20,20,20,20,20,0,20,0,20,20,20,20,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"f":"````````````````{b{{h{df}}}}{j{{h{df}}}}{{}l}{{{A`{}{{n{c}}}}AbAdAf}{{h{cf}}}j}{{{A`{}{{n{c}}}}AbAdAf}{{h{{Ah{c}}f}}}j}{bl}```{ce{}{}}0{AjAj}{{ce}d{}{}}{AlAb}{AjAb}{AlAj}{{AjAn}B`}{cc{}}{{BbAb}Aj}{{BdBfAd}{{h{Bhf}}}}{{BdBfAdAb}{{h{Bhf}}}}:{AlBf}{AjBb}{{AjAd}{{h{Abf}}}}{{AjBfAd}{{h{Bhf}}}}>{c{{h{e}}}{}{}}0{cBj{}}``````````{{c{h{ef}}}{{h{ef}}}Bd{}}{{fBb}d}{ce{}{}}000{fBl}{{{C`{Bn}}{C`{Cb}}}Bl}{{BlAn}B`}0{{fAn}B`}0{{c{h{eg}}km}{{h{eg}}}Cd{}{}{}{{Ch{}{{Cf{{h{ig}}}}}}}{{Ch{ei}{{Cf{{h{eg}}}}}}}}{cc{}}{{BbBl}f}1{Cjf}{{BbAdAd}f}99{fl}0{{fBf}l}11{fBb}{{cg}{{h{de}}}Cd{}{{Ch{}{{Cf{{h{de}}}}}}}}{cBb{}}0{c{{h{e}}}{}{}}000{cBj{}}0```````{{{Ah{Cl}}BfBf}{{h{df}}}}{{Af{C`{Af}}}{{h{Cnf}}}}{AfCn}`````{ce{}{}}0{BhBh}{{ce}d{}{}}{{BhBh}l}{{BhAn}B`}{cc{}}{{{Db{{D`{BbBh}}}}}Ab}6{{BhAd}{{h{Bhf}}}}7<<;{{Ad{h{cf}}gi}{{h{cf}}}{}{}{{Ch{BbAd}{{Cf{{h{ef}}}}}}}{{Ch{ce}{{Cf{{h{cf}}}}}}}}{{BfAd}{{h{lf}}}}{{BfAbAb}{{h{lf}}}}{{BfAdAbAb}{{h{lf}}}}{{BfAd}{{h{Ddf}}}}{{BfAbAb}{{h{Ddf}}}}{{BfAdAbAb}{{h{Ddf}}}}{{BfAd}{{h{{Ah{Bb}}f}}}}{{BfAbAb}{{h{Bhf}}}}{{BfBhAd}{{h{Bhf}}}}2{{Ad{C`{Bf}}}{{h{Bbf}}}}{{BfAd}{{h{Bbf}}}}{{BfAbAb}{{h{Bbf}}}}{{BfAdAbAb}{{h{Bbf}}}}{{Ade}{{h{{Ah{c}}f}}}{}{{Ch{Ad}{{Cf{{h{cf}}}}}}}}{Ad{{h{{Ah{Ad}}f}}}}{Ad{{h{lf}}}}{Ad{{h{Ddf}}}}{Ad{{h{Bbf}}}}{Ad{{h{{Ah{Bb}}f}}}}{{BfAde}{{h{cf}}}{}{{Ch{Ad}{{Cf{{h{cf}}}}}}}}{{Adc}{{h{df}}}{{Ch{BbAd}{{Cf{{h{df}}}}}}}}{Af{{h{{Ah{Ad}}f}}}}","c":[],"p":[[10,"ConditionalAction",11],[1,"unit"],[5,"DotfilesError",46],[6,"Result",144],[10,"Action",11],[1,"bool"],[17,"ActionType"],[10,"ActionParser",11],[8,"Settings",101],[6,"StrictYaml",145],[5,"Path",146],[5,"Vec",147],[5,"DirectiveData",22],[10,"HasDirectiveData",22],[5,"Formatter",148],[8,"Result",148],[5,"String",149],[10,"Directive",22],[1,"str"],[6,"Setting",101],[5,"TypeId",150],[6,"ErrorType",46],[6,"PopenError",151],[6,"Option",152],[6,"ExitStatus",153],[10,"IntoIterator",154],[17,"Output"],[10,"FnMut",155],[5,"Error",156],[5,"Exec",157],[5,"PathBuf",146],[1,"tuple"],[1,"slice"],[1,"i64"],[15,"UnexpectedYamlTypeError",91],[15,"ExecutionError",91],[15,"FileSystemError",91],[15,"IncompleteConfigurationError",91],[15,"YamlParseError",91]],"b":[[64,"impl-Debug-for-ErrorType"],[65,"impl-Display-for-ErrorType"],[66,"impl-Display-for-DotfilesError"],[67,"impl-Debug-for-DotfilesError"]]}],\
["dotfiles_core_macros",{"doc":"This crate provides procedural macros to generate …","t":"YY","n":["ConditionalAction","Directive"],"q":[[0,"dotfiles_core_macros"]],"d":["Generates an implementation of ConditionalAction&lt;’a&gt; …","Generates a Directive&lt;’a&gt; and a HasDirectiveData&lt;’a&gt; …"],"i":[0,0],"f":"``","c":[],"p":[],"b":[]}],\
["dotfiles_processor",{"doc":"","t":"CCCCFNNNNNNNNNNNNNNNNNPGPPFPPPGPPPNNNNNNNNNNNNNNONNNNNNNNNNNNNNNONNNNNNNNNNNNNNNNNNNNOOOPPPPPPPPGGPPPPNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNH","n":["context","flags","known_directive","process","Context","actions","borrow","borrow_mut","defaults","file","from","get_default","into","parse_file","run_actions","subcontext","try_from","try_from","try_from","try_into","type_id","vzip","ApplyConfig","Command","Debug","Error","FlagData","Info","InstallHomebrew","InstallOhMyZsh","LogLevelFilter","Off","Trace","Warn","augment_args","augment_args_for_update","augment_subcommands","augment_subcommands_for_update","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","cmp","command","command","command_for_update","eq","from","from","from","from_arg_matches","from_arg_matches","from_arg_matches_mut","from_arg_matches_mut","group_id","has_subcommand","into","into","into","into","log_level_filter","partial_cmp","to_owned","to_possible_value","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches_mut","update_from_arg_matches_mut","value_variants","vzip","vzip","vzip","dry_run","file","skip_chsh","Apt","Apt","Brew","Brew","Create","Create","Exec","Exec","KnownAction","KnownDirective","Link","Link","Subconfig","Subconfig","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","data","execute","from","from","from","from","from","from","from","from","into","into","parse_action_list","parse_context_defaults","to_owned","try_from","try_from","try_from","try_into","try_into","type_id","type_id","vzip","vzip","process"],"q":[[0,"dotfiles_processor"],[4,"dotfiles_processor::context"],[22,"dotfiles_processor::flags"],[85,"dotfiles_processor::flags::Command"],[88,"dotfiles_processor::known_directive"],[132,"dotfiles_processor::process"],[133,"alloc::vec"],[134,"alloc::string"],[135,"dotfiles_core::settings"],[136,"std::collections::hash::map"],[137,"std::path"],[138,"dotfiles_core::settings"],[139,"dotfiles_core::error"],[140,"core::result"],[141,"std::path"],[142,"clap_builder::builder::command"],[143,"core::cmp"],[144,"clap_builder::parser::matches::arg_matches"],[145,"clap_builder"],[146,"clap_builder::util::id"],[147,"log"],[148,"clap_builder::builder::possible_value"],[149,"dotfiles_core::directive"],[150,"dotfiles_actions::create::action"],[151,"dotfiles_actions::link::action"],[152,"dotfiles_actions::apt::action"],[153,"dotfiles_actions::exec::action"],[154,"dotfiles_actions::brew::action"],[155,"strict_yaml_rust::strict_yaml"]],"d":["","","","","A context represents an environment in which defaults can …","The list of actions parsed from this file.","","","The default overrides for the current context.","The absolute path to the file to which this context …","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","Runs the actions in this context and consumes the context.","","","","","","","","Applies the configuration in the file passed as argument.","","Corresponds to the <code>Debug</code> log level.","Corresponds to the <code>Error</code> log level.","","Corresponds to the <code>Info</code> log level.","Installs Homebrew on Mac or Linuxbrew on Linux, see …","Installs Oh My Zsh! and sets <code>zsh</code> as the shell, see …","","A level lower than all log levels.","Corresponds to the <code>Trace</code> log level.","Corresponds to the <code>Warn</code> log level.","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","Only parse the configuration, do not run it.","Configuration file that describes what to do.","Skips running <code>chsh</code> to set <code>zsh</code> as the shell","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","",""],"i":[0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,23,0,17,17,0,17,23,23,0,17,17,17,21,21,23,23,21,17,23,21,17,23,17,17,17,21,21,21,17,21,17,23,21,23,21,23,21,23,21,17,17,23,21,17,17,17,21,17,23,21,17,23,21,17,23,21,23,21,23,17,21,17,23,37,37,38,2,28,2,28,2,28,2,28,0,0,2,28,2,28,2,28,2,28,28,28,28,2,2,2,2,2,2,2,2,28,2,28,28,28,28,2,28,28,2,28,2,28,2,28,0],"f":"`````{b{{f{d}}}}{ce{}{}}0{b{{l{hj}}}}{bn}{cc{}}{{bA`A`}{{Ad{Ab}}}}4{b{{Aj{AfAh}}}}0{{bAl}{{Aj{bAh}}}}{c{{Aj{e}}}{}{}}{A`{{Aj{bc}}}{}}{Al{{Aj{bc}}}{}}2{cAn{}}:````````````{B`B`}000;;;;;;{BbBb}{{ce}Af{}{}}{{BbBb}Bd}{{}B`}`0{{BbBb}Bf}==={Bh{{Aj{BjBl}}}}{Bh{{Aj{BnBl}}}}10{{}{{Ad{C`}}}}{A`Bf}{ce{}{}}0{BbCb}1`{{BbBb}{{Ad{Bd}}}}2{Bb{{Ad{Cd}}}}{c{{Aj{e}}}{}{}}00000???{{BjBh}{{Aj{AfBl}}}}{{BnBh}{{Aj{AfBl}}}}10{{}{{Cf{Bb}}}}777`````````````````7777{ChCh}{{ce}Af{}{}}{ChCj}{d{{Aj{AfAh}}}}{Cld}{cc{}}{Cnd}{D`d}{Dbd}{Ddd}{bd}5{ce{}{}}0{{ChjDfb}{{Aj{{f{d}}Ah}}}}{{ChDf}{{Aj{{Dh{hj}}Ah}}}}2{c{{Aj{e}}}{}{}}0{A`{{Aj{Chc}}}{}}11{cAn{}}055{Bj{{Aj{AfAh}}}}","c":[],"p":[[5,"Context",4],[6,"KnownAction",88],[5,"Vec",133],[5,"String",134],[8,"Settings",135],[5,"HashMap",136],[5,"PathBuf",137],[1,"str"],[6,"Setting",135],[6,"Option",138],[1,"unit"],[5,"DotfilesError",139],[6,"Result",140],[5,"Path",137],[5,"TypeId",141],[5,"Command",142],[6,"LogLevelFilter",22],[6,"Ordering",143],[1,"bool"],[5,"ArgMatches",144],[5,"FlagData",22],[8,"Error",145],[6,"Command",22],[5,"Id",146],[6,"LevelFilter",147],[5,"PossibleValue",148],[1,"slice"],[6,"KnownDirective",88],[5,"DirectiveData",149],[8,"NativeCreateAction",150],[8,"NativeLinkAction",151],[5,"AptAction",152],[5,"ExecAction",153],[5,"BrewAction",154],[6,"StrictYaml",155],[1,"tuple"],[15,"ApplyConfig",85],[15,"InstallOhMyZsh",85]],"b":[[17,"impl-TryFrom%3C%26str%3E-for-Context"],[18,"impl-TryFrom%3C%26Path%3E-for-Context"],[110,"impl-From%3CCreateAction%3C\'a,+OsFileSystem%3E%3E-for-KnownAction%3C\'a%3E"],[112,"impl-From%3CLinkAction%3C\'a,+OsFileSystem%3E%3E-for-KnownAction%3C\'a%3E"],[113,"impl-From%3CAptAction%3C\'a%3E%3E-for-KnownAction%3C\'a%3E"],[114,"impl-From%3CExecAction%3C\'a%3E%3E-for-KnownAction%3C\'a%3E"],[115,"impl-From%3CBrewAction%3C\'a%3E%3E-for-KnownAction%3C\'a%3E"],[116,"impl-From%3CContext%3E-for-KnownAction%3C\'a%3E"]]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
