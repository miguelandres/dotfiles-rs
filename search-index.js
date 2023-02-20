var searchIndex = JSON.parse('{\
"dotfiles":{"doc":"","t":"F","n":["main"],"q":["dotfiles"],"d":[""],"i":[0],"f":[[[]]],"p":[]},\
"dotfiles_actions":{"doc":"This crate contains all concrete actions and directives. …","t":"AAAAAAAAADLLLLLLLLLLLLLLLLLLDRRRRRLLLLLLLFLLLLLLLLAADGGLLLLLLLLLLLLLLLLRDRRGGLLLLLLLLFLLLLLLLLAADLLLLLLLLLLLLLLLLRRRRDLLLLLLLFLLLLLLLIKKADLLLLLLLLLLLLAAGDGLLLLLLLLLLLLLLLLLLLLRRRGRDGRRRRLLLLLLLLLFLLLLLLLLLADLLLLLLLLLLLL","n":["brew","create","exec","filesystem","homebrew_install","link","ohmyzsh_install","action","directive","BrewAction","borrow","borrow_mut","casks","eq","execute","fmt","force_casks","formulae","from","into","new","skip_in_ci","skip_in_ci","taps","try_from","try_into","type_id","vzip","BrewDirective","CASK_SETTING","DIRECTIVE_NAME","FORCE_CASKS_SETTING","FORMULA_SETTING","TAP_SETTING","borrow","borrow_mut","clone","clone_into","default","directive_data","from","init_directive_data","into","parse_action","parse_action_list","to_owned","try_from","try_into","type_id","vzip","action","directive","CreateAction","FakeCreateAction","NativeCreateAction","borrow","borrow_mut","create_parent_dirs","current_dir","directory","eq","execute","fmt","from","into","new","skip_in_ci","try_from","try_into","type_id","vzip","CREATE_PARENT_DIRS_SETTING","CreateDirective","DIRECTIVE_NAME","DIR_SETTING","FakeCreateDirective","NativeCreateDirective","borrow","borrow_mut","clone","clone_into","default","directive_data","from","fs","init_directive_data","into","mut_fs","parse_action","to_owned","try_from","try_into","type_id","vzip","action","directive","ExecAction","borrow","borrow_mut","command","description","echo","eq","execute","fmt","from","into","new","skip_in_ci","try_from","try_into","type_id","vzip","COMMAND_SETTING","DESCRIPTION_SETTING","DIRECTIVE_NAME","ECHO_SETTING","ExecDirective","borrow","borrow_mut","clone","clone_into","default","directive_data","from","init_directive_data","into","parse_action","to_owned","try_from","try_into","type_id","vzip","FileSystemDirective","fs","mut_fs","action","HomebrewInstallAction","borrow","borrow_mut","check_brew_is_installed","default","execute","from","into","new","try_from","try_into","type_id","vzip","action","directive","FakeLinkAction","LinkAction","NativeLinkAction","borrow","borrow_mut","create_parent_dirs","eq","execute","fmt","force","from","ignore_missing_target","into","new","path","relink","resolve_symlink_target","skip_in_ci","target","try_from","try_into","type_id","vzip","CREATE_PARENT_DIRS_SETTING","DIRECTIVE_NAME","FORCE_SETTING","FakeLinkDirective","IGNORE_MISSING_TARGET_SETTING","LinkDirective","NativeLinkDirective","PATH_SETTING","RELINK_SETTING","RESOLVE_SYMLINK_TARGET_SETTING","TARGET_SETTING","borrow","borrow_mut","clone","clone_into","default","directive_data","from","fs","fs","init_directive_data","into","mut_fs","parse_action","parse_shortened_action","to_owned","try_from","try_into","type_id","vzip","action","OhMyZshInstallAction","borrow","borrow_mut","check_oh_my_zsh_is_installed","check_zsh_is_installed","execute","from","into","new","try_from","try_into","type_id","vzip"],"q":["dotfiles_actions","","","","","","","dotfiles_actions::brew","","dotfiles_actions::brew::action","","","","","","","","","","","","","","","","","","","dotfiles_actions::brew::directive","","","","","","","","","","","","","","","","","","","","","","dotfiles_actions::create","","dotfiles_actions::create::action","","","","","","","","","","","","","","","","","","","dotfiles_actions::create::directive","","","","","","","","","","","","","","","","","","","","","","","dotfiles_actions::exec","","dotfiles_actions::exec::action","","","","","","","","","","","","","","","","","dotfiles_actions::exec::directive","","","","","","","","","","","","","","","","","","","","dotfiles_actions::filesystem","","","dotfiles_actions::homebrew_install","dotfiles_actions::homebrew_install::action","","","","","","","","","","","","","dotfiles_actions::link","","dotfiles_actions::link::action","","","","","","","","","","","","","","","","","","","","","","","dotfiles_actions::link::directive","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","dotfiles_actions::ohmyzsh_install","dotfiles_actions::ohmyzsh_install::action","","","","","","","","","","","",""],"d":["This module contains the BrewAction and BrewDirective","This module contains the CreateAction and CreateDirective …","This module contains the ExecAction and ExecDirective used …","Module that contains interfaces common to directives that …","This module contains the HomebrewInstallAction","This module contains the LinkAction and LinkDirective …","This module contains the OhMyZshInstallAction","This module contains the BrewAction that installs a brew …","This module defines BrewDirective.","BrewAction Installs software using homebrew.","","","List of casks to install. Casks usually are macOS apps …","","","","Passes <code>--force</code> to <code>brew install --cask</code> to prevent the …","List of brew formulae to <code>brew install</code>, usually command …","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Constructs a new BrewAction","Skips this action if it is running in a CI environment.","","List of repositories to tap into using <code>brew tap</code>.","","","","","A directive that can build BrewActions to install …","The string that identifies the list of casks to install","Name of the Brew directive","force casks to deal with previously installed apps","The string that identifies the list of formulae to install","The string that identifies the list of taps to install","","","","","","","Returns the argument unchanged.","Initialize the defaults for the BrewDirective.","Calls <code>U::from(self)</code>.","","Parse the list of actions from yaml, in this case it’s …","","","","","","This module contains the CreateAction that creates a new …","This module defines CreateDirective.","CreateAction creates a new directory when executed","A Fake create action that works on a fake test filesystem.","A native create action that works on the real filesystem.","","","Force creation of the directory and all its parents if …","Current directory that will be used to determine relative …","Directory to create. Can be absolute or relative.","","Creates the <code>directory</code>.","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Constructs a new instance of CreateAction","","","","","","Constant for the name of the <code>create_parent_dirs</code> Setting …","A directive that can build CreateActions to create …","Constant for the name of the <code>create</code> directive.","Constant for the name of the <code>directory</code> argument that …","CreateDirective that uses the native FakeFileSystem for …","CreateDirective that uses the native OsFileSystem.","","","","","","","Returns the argument unchanged.","","Initializes the default configuration for the …","Calls <code>U::from(self)</code>.","","","","","","","","This module contains the ExecAction that executes a …","This module defines ExecDirective which represents …","ExecAction Installs software using homebrew.","","","The command to run","Description for the command to run.","Whether to print out the command for clarity.","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Create a new Exec Action that will run from the parent …","","","","","","Command to run","Optional description for the command to run","Name of the Exec directive","Echo the command to run before running it.","A directive that can build ExecActions to run commands","","","","","","","Returns the argument unchanged.","Initialize the defaults for the BrewDirective.","Calls <code>U::from(self)</code>.","","","","","","","Common trait for all the directives that use a Filesystem","Returns the filesystem instance","Returns a mutable reference to the filesystem instance","This module contains the HomebrewInstallAction that …","HomebrewInstallAction installs homebrew.","","","Returns true if it can find a brew command","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Constructs a new HomebrewInstallAction","","","","","This module contains the LinkAction that creates a new …","This module defines LinkDirective.","A Fake create action that works on a fake test filesystem.","LinkAction creates a new symlink <code>path</code> that points to <code>target</code>…","A native create action that works on the real filesystem.","","","Create all parent directories if they do not exist already","","","","Force to replace an existing file or directory when …","Returns the argument unchanged.","Succeed even if <code>target</code> doesn’t point to an existing file …","Calls <code>U::from(self)</code>.","Constructs a new LinkAction","Path of the new symlink","Force to re-create the symlink if it exists already","If the target is another symlink, resolve the ultimate …","","Path that the symlink points to.","","","","","Create parent dirs if they don’t exist","Name of the link directive","Force setting, replaces any other file or directory","LinkDirective that uses the native FakeFileSystem for …","Create the symlink even if the target file does not exist","A directive that can build LinkActions to create …","LinkDirective that uses the native OsFileSystem.","Path setting (path of the symlink)","Relink setting, if true the action relinks an existing …","Resolves the target if it is a symlink and uses the final …","Target setting (path to the file the symlink points to)","","","","","","","Returns the argument unchanged.","","Returns the FileSystem instance being used.","Initialize the defaults for the LinkDirective.","Calls <code>U::from(self)</code>.","","","Parse a shortened action with only link name to target name","","","","","","This module contains the OhMyZshInstallAction that sets up …","OhMyZshInstallAction sets up ohmyzsh.","","","Returns true if the $ZSH environment var is set","Returns true if it can find a brew command","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Constructs a new OhMyZshInstallAction","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,10,10,10,10,10,10,10,0,10,10,10,10,10,10,10,10,0,0,0,0,0,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,0,0,0,0,0,0,20,20,20,20,20,20,20,20,0,20,20,20,20,20,20,20,20,0,0,0,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,22,0,0,0,0,0,25,25,25,25,25,25,25,0,25,25,25,25,25,25,25,0,31,31,0,0,26,26,26,26,26,26,26,26,26,26,26,26,0,0,0,0,0,28,28,28,28,28,28,28,28,28,28,28,28,28,28,28,28,28,28,28,28,0,0,0,0,0,0,0,0,0,0,0,29,29,29,29,29,29,29,29,29,0,29,29,29,29,29,29,29,29,29,0,0,30,30,30,30,30,30,30,30,30,30,30,30],"f":[0,0,0,0,0,0,0,0,0,0,[[]],[[]],[1,2],[[1,1],3],[1,[[5,[4]]]],[[1,6],7],[1,3],[1,2],[[]],[[]],[[3,3,[2,[8]],[2,[8]],[2,[8]]],1],[1,3],[1,3],[1,2],[[],5],[[],5],[[],9],[[]],0,0,0,0,0,0,[[]],[[]],[10,10],[[]],[[],10],[10,11],[[]],[[],11],[[]],[[10,12,13,14],[[5,[1,4]]]],[[10,12,13,14],[[5,[[2,[1]],4]]]],[[]],[[],5],[[],5],[[],9],[[]],0,0,0,0,0,[[]],[[]],[[[16,[15]]],3],[[[16,[15]]],17],[[[16,[15]]],8],[[[16,[15]],[16,[15]]],3],[[[16,[15]]],[[5,[4]]]],[[[16,[15]],6],7],[[]],[[]],[[3,8,3,17],[[5,[[16,[15]],4]]]],[[[16,[15]]],3],[[],5],[[],5],[[],9],[[]],0,0,0,0,0,0,[[]],[[]],[[[20,[[0,[18,15,19]]]]],[[20,[[0,[18,15,19]]]]]],[[]],[[],[[20,[[0,[15,19]]]]]],[[[20,[[0,[15,19]]]]],11],[[]],[[[20,[[0,[15,19]]]]]],[[],11],[[]],[[[20,[[0,[15,19]]]]]],[[[20,[[0,[15,19]]]],21,13,14],[[5,[[16,[[0,[15,19]]]],4]]]],[[]],[[],5],[[],5],[[],9],[[]],0,0,0,[[]],[[]],[22,23],[22,[[24,[8]]]],[22,3],[[22,22],3],[22,[[5,[4]]]],[[22,6],7],[[]],[[]],[[3,8,[24,[8]],3,14],[[5,[22,4]]]],[22,3],[[],5],[[],5],[[],9],[[]],0,0,0,0,0,[[]],[[]],[25,25],[[]],[[],25],[25,11],[[]],[[],11],[[]],[[25,21,13,14],[[5,[22,4]]]],[[]],[[],5],[[],5],[[],9],[[]],0,[[]],[[]],0,0,[[]],[[]],[26,3],[[],26],[26,[[5,[4]]]],[[]],[[]],[[],26],[[],5],[[],5],[[],9],[[]],0,0,0,0,0,[[]],[[]],[[[28,[[0,[15,27]]]]],3],[[[28,[[0,[15,27]]]],[28,[[0,[15,27]]]]],3],[[[28,[[0,[15,27]]]]],[[5,[4]]]],[[[28,[[0,[15,27]]]],6],7],[[[28,[[0,[15,27]]]]],3],[[]],[[[28,[[0,[15,27]]]]],3],[[]],[[8,8,12,12,17],[[5,[[28,[[0,[15,27]]]],4]]]],[[[28,[[0,[15,27]]]]],8],[[[28,[[0,[15,27]]]]],3],[[[28,[[0,[15,27]]]]],3],[[[28,[[0,[15,27]]]]],3],[[[28,[[0,[15,27]]]]],8],[[],5],[[],5],[[],9],[[]],0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[[29,[[0,[18,15,27,19]]]]],[[29,[[0,[18,15,27,19]]]]]],[[]],[[],[[29,[[0,[15,27,19]]]]]],[[[29,[[0,[15,27,19]]]]],11],[[]],[[[29,[[0,[15,27,19]]]]]],[[[29,[[0,[15,27,19]]]]]],[[],11],[[]],[[[29,[[0,[15,27,19]]]]]],[[[29,[[0,[15,27,19]]]],12,13,14],[[5,[[28,[[0,[15,27,19]]]],4]]]],[[[29,[[0,[15,27,19]]]],12,13,14],[[5,[[28,[[0,[15,27,19]]]],4]]]],[[]],[[],5],[[],5],[[],9],[[]],0,0,[[]],[[]],[30,3],[30,3],[30,[[5,[4]]]],[[]],[[]],[3,30],[[],5],[[],5],[[],9],[[]]],"p":[[3,"BrewAction"],[3,"Vec"],[15,"bool"],[3,"DotfilesError"],[4,"Result"],[3,"Formatter"],[6,"Result"],[3,"String"],[3,"TypeId"],[3,"BrewDirective"],[3,"DirectiveData"],[6,"Settings"],[4,"StrictYaml"],[3,"Path"],[8,"FileSystem"],[3,"CreateAction"],[3,"PathBuf"],[8,"Clone"],[8,"Default"],[3,"CreateDirective"],[3,"HashMap"],[3,"ExecAction"],[15,"str"],[4,"Option"],[3,"ExecDirective"],[3,"HomebrewInstallAction"],[8,"UnixFileSystem"],[3,"LinkAction"],[3,"LinkDirective"],[3,"OhMyZshInstallAction"],[8,"FileSystemDirective"]]},\
"dotfiles_core":{"doc":"The core of Dotfiles-rs is basically a set of directives …","t":"CCCCAAAAAAAIIQIRLKFKLKIDILLLLLLKLLLLLLLLLLLLLLNDENNNNNNNFLLLLLLFLLLLFLLLLLLLLLLLLLFLLLLLLLLMMMMMMMFFFNNEGNLLLLLLLFLFLLLLFFFFFFFFFFFFFFFFFFFFFFFF","n":["Action","Directive","Setting","Settings","action","directive","error","exec_wrapper","path","settings","yaml_util","Action","ActionParser","ActionType","ConditionalAction","SKIP_IN_CI_SETTING","check_conditions_and_execute","execute","is_running_in_ci","parse_action","parse_action_list","skip_in_ci","Directive","DirectiveData","HasDirectiveData","borrow","borrow_mut","clone","clone_into","defaults","defaults","directive_data","fmt","from","from","get_setting_from_yaml_hash","get_setting_from_yaml_hash_or_from_context","into","name","name","parse_context_defaults","parse_setting_value","to_owned","try_from","try_into","type_id","CoreError","DotfilesError","ErrorType","ExecutionError","FileSystemError","IncompleteConfigurationError","InconsistentConfigurationError","TestingErrorActionSucceedsWhenItShouldFail","UnexpectedYamlTypeError","YamlParseError","add_directive_error_prefix","add_message_prefix","borrow","borrow","borrow_mut","borrow_mut","error_type","execution_error","fmt","fmt","fmt","fmt","fold_until_first_err","from","from","from","from_io_error","from_wrong_yaml","into","into","is_fs_error","is_inconsistent_config","is_missing_config","is_wrong_yaml","is_yaml_parse_error","message","process_until_first_err","to_string","to_string","try_from","try_from","try_into","try_into","type_id","type_id","encountered","exit_status","expected","fs_error","missing_field","popen_error","scan_error","execute_commands","convert_path_to_absolute","process_home_dir_in_path","Boolean","Integer","Setting","Settings","String","borrow","borrow_mut","clone","clone_into","eq","fmt","from","initialize_settings_object","into","parse_setting","to_owned","try_from","try_into","type_id","fold_hash_until_first_err","get_boolean_from_yaml_hash","get_boolean_setting_from_context","get_boolean_setting_from_yaml_or_context","get_integer_from_yaml_hash","get_integer_setting","get_integer_setting_from_yaml_or_context","get_optional_string_array_from_yaml_hash","get_setting_from_context","get_setting_from_yaml_hash","get_string_array_from_yaml_hash","get_string_content_or_keyed_value","get_string_from_yaml_hash","get_string_setting","get_string_setting_from_yaml_or_context","map_yaml_array","parse_as_array","parse_as_boolean","parse_as_integer","parse_as_string","parse_as_string_array","process_value_from_yaml_hash","process_yaml_hash_until_first_err","read_yaml_file"],"q":["dotfiles_core","","","","","","","","","","","dotfiles_core::action","","","","","","","","","","","dotfiles_core::directive","","","","","","","","","","","","","","","","","","","","","","","","dotfiles_core::error","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","dotfiles_core::error::ErrorType","","","","","","","dotfiles_core::exec_wrapper","dotfiles_core::path","","dotfiles_core::settings","","","","","","","","","","","","","","","","","","","dotfiles_core::yaml_util","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","This module contains the base trait for all Actions.","This module contains the base trait for all Directive and …","Module for the error handling classes and enums.","Wraps some logic to run external commands and handle errors","Contains helpful functions to deal with paths in the …","This module contains the definition of a setting and code …","Module that defines helper functions to process YAML …","An action to be run by a the dotfiles runtime.","Trait to parse a specific action type from StrictYaml.","The action type this object parses","Trait for actions to be skippable under certain conditions.","Skip this whole action in CI environments.","Checks that the conditions allow for executing this …","Executes the action.","Whether the execution environment is presumed to be CI","Builds a single action of type ActionParser::ActionType …","Builds a list of actions of type ActionParser::ActionType …","Whether to skip this action in Continuous Integration …","A parser for action steps, each directive represents a …","A struct that contains the default settings for a …","A trait for all directives, it is shared between …","","","","","Returns the default settings as configured.","Default settings for this directive.","Returns the directive data for this object","","Constructs a new directive from a name and a set of …","Returns the argument unchanged.","Parses an individual setting named <code>name</code> from a yaml hash …","Parse a particular setting with its correct type from …","Calls <code>U::from(self)</code>.","Returns the name of the directive.","Unique name of this directive.","Parses all settings for this directive from StrictYaml, …","<code>DirectiveData.setting_types</code>.","","","","","A core logic error for Dotfiles-rs","Struct that represents an error that happened while …","A collection of types of errors that may occur while …","An error occurred while running a command necessary for …","A filesystem error that was encountered while either …","The configuration is missing a required field","The configuration file is inconsistent with itself or with …","An error only for testing, the action that should fail …","Received an StrictYaml object of an unexpected type","An error that occurred while parsing the StrictYaml file","Adds a prefix to an error with the name of the directive …","Adds a prefix to the existing message","","","","","Error type","Creates an ErrorType::ExecutionError","","","","","Executes the <code>process_function</code> on each of the items in the …","Returns the argument unchanged.","Creates a new Dotfiles error with the given message and …","Returns the argument unchanged.","Creates a new Dotfiles error with the given message and …","Creates a new Dotfiles error with the given message and …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns whether the error is a Fs error.","Returns whether the error is an Inconsistent Config.","returns whether the underlying error is a missing …","Returns whether the error is a wrong yaml type.","Returns whether the error is a wrong yaml type.","Human-readable error message","Executes the <code>process_function</code> on each of the items in the …","","","","","","","","","What we got instead of the expected type.","If the command attempted to execute but failed for some …","An example of what we expected.","The underlying filesystem error.","Name of the field missing in the configuration","If the command could not execute for some reason the …","The underlying scan error","Executes the <code>cmd</code> and waits for it to finish.","Converts a file path to absolute if it is relative. If …","Checks for ~ and replaces it with a home directory if …","A boolean value for a setting","An Integer value for a setting","Represents a value for a setting","The Settings object is a hashmap of option names to a …","A string value for a setting","","","","","","","Returns the argument unchanged.","Returns a Settings object from an array as a bit of …","Calls <code>U::from(self)</code>.","Parse a setting from StrictYaml given a particular setting …","","","","","Process each element of the hash with the <code>process_function</code> …","Gets a specific boolean setting from a yaml hash","Gets a boolean value for the setting named <code>name</code>.","Gets a Boolean value from YAML or context.","Gets a specific integer setting from a yaml hash","Gets a Int value for the setting named <code>name</code>.","Gets a Integer value from YAML or context.","Gets a specific string array setting from a yaml hash, but …","Gets a String value for the setting named <code>name</code>.","Gets a specific setting from a yaml hash","Gets a specific string array setting from a yaml hash","Gets the content of this YAML node or the value for a …","Gets a specific string setting from a yaml hash","Gets a String value for the setting named <code>name</code>.","Gets a String value from YAML or context.","Calls a processing function on all elements of an array, …","Parse a yaml element as an array.","Parse a yaml element as boolean.","Parse a yaml element as Integer.","Parse a yaml element as string, will convert booleans and …","Gets a native <code>Vec&lt;String&gt;</code> from a StrictYaml::Array. It …","Gets the value for a specified key in a yaml hash and does …","Executes the <code>process_function</code> on each of the items in the …","Reads a StrictYaml File. Returns Error in case of a syntax …"],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,23,0,0,24,25,0,23,23,24,0,0,0,8,8,8,8,26,8,26,8,8,8,27,27,8,26,8,8,8,8,8,8,8,15,0,0,15,15,15,15,15,15,15,0,1,15,1,15,1,1,0,15,15,1,1,0,15,1,1,1,1,15,1,1,1,1,1,1,1,0,15,1,15,1,15,1,15,1,28,29,28,30,31,29,32,0,0,0,13,13,0,0,13,13,13,13,13,13,13,13,0,13,0,13,13,13,13,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[],[[2,[1]]]],[[],[[2,[1]]]],[[],3],[[4,5,6],[[2,[1]]]],[[4,5,6],[[2,[7,1]]]],[[],3],0,0,0,[[]],[[]],[8,8],[[]],[[],4],[8,4],[[],8],[[8,9],10],[[11,4],8],[[]],[[12,5],[[2,[13,1]]]],[[12,5,4],[[2,[13,1]]]],[[]],[[],12],[8,11],[[8,5],[[2,[4,1]]]],[[8,12,5],[[2,[13,1]]]],[[]],[[],2],[[],2],[[],14],0,0,0,0,0,0,0,0,0,0,[[[2,[1]]],[[2,[1]]]],[[1,11]],[[]],[[]],[[]],[[]],[1,15],[[[17,[16]],[17,[18]]],15],[[15,9],10],[[15,9],10],[[1,9],10],[[1,9],10],[2,2],[[]],[[11,15],1],[[]],[19,1],[[11,5,5],1],[[]],[[]],[1,3],[1,3],[[1,12],3],[1,3],[1,3],[1,11],[[],2],[[],11],[[],11],[[],2],[[],2],[[],2],[[],2],[[],14],[[],14],0,0,0,0,0,0,0,[[[7,[20]],12,12],[[2,[1]]]],[[6,[17,[6]]],[[2,[21,1]]]],[6,21],0,0,0,0,0,[[]],[[]],[13,13],[[]],[[13,13],3],[[13,9],10],[[]],[[],4],[[]],[[13,5],[[2,[13,1]]]],[[]],[[],2],[[],2],[[],14],[[5,[2,[1]]],[[2,[1]]]],[[12,5],[[2,[3,1]]]],[[12,4,4],[[2,[3,1]]]],[[12,5,4,4],[[2,[3,1]]]],[[12,5],[[2,[22,1]]]],[[12,4,4],[[2,[22,1]]]],[[12,5,4,4],[[2,[22,1]]]],[[12,5],[[2,[[7,[11]],1]]]],[[12,4,4],[[2,[13,1]]]],[[12,13,5],[[2,[13,1]]]],[[12,5],[[2,[[7,[11]],1]]]],[[5,[17,[12]]],[[2,[11,1]]]],[[12,5],[[2,[11,1]]]],[[12,4,4],[[2,[11,1]]]],[[12,5,4,4],[[2,[11,1]]]],[5,[[2,[7,1]]]],[5,[[2,[[7,[5]],1]]]],[5,[[2,[3,1]]]],[5,[[2,[22,1]]]],[5,[[2,[11,1]]]],[5,[[2,[[7,[11]],1]]]],[[12,5],[[2,[1]]]],[5,[[2,[1]]]],[6,[[2,[[7,[5]],1]]]]],"p":[[3,"DotfilesError"],[4,"Result"],[15,"bool"],[6,"Settings"],[4,"StrictYaml"],[3,"Path"],[3,"Vec"],[3,"DirectiveData"],[3,"Formatter"],[6,"Result"],[3,"String"],[15,"str"],[4,"Setting"],[3,"TypeId"],[4,"ErrorType"],[4,"PopenError"],[4,"Option"],[4,"ExitStatus"],[3,"Error"],[3,"Exec"],[3,"PathBuf"],[15,"i64"],[8,"ActionParser"],[8,"ConditionalAction"],[8,"Action"],[8,"HasDirectiveData"],[8,"Directive"],[13,"UnexpectedYamlTypeError"],[13,"ExecutionError"],[13,"FileSystemError"],[13,"IncompleteConfigurationError"],[13,"YamlParseError"]]},\
"dotfiles_core_macros":{"doc":"This crate provides procedural macros to generate …","t":"YY","n":["ConditionalAction","Directive"],"q":["dotfiles_core_macros",""],"d":["Generates an implementation of ConditionalAction&lt;’a&gt; …","Generates a Directive&lt;’a&gt; and a HasDirectiveData&lt;’a&gt; …"],"i":[0,0],"f":[0,0],"p":[]},\
"dotfiles_processor":{"doc":"","t":"AAAADLLLLLLLLLLLLLLLLLNENNDNNNENNNLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLMMMNNNNNNEENNNNLLLLLLLLLLLLLLLLLLLLLLLLLLLLLF","n":["context","flags","known_directive","process","Context","actions","borrow","borrow_mut","defaults","file","from","get_default","into","parse_file","run_actions","subcontext","try_from","try_from","try_from","try_into","type_id","vzip","ApplyConfig","Command","Debug","Error","FlagData","Info","InstallHomebrew","InstallOhMyZsh","LogLevelFilter","Off","Trace","Warn","augment_args","augment_args_for_update","augment_subcommands","augment_subcommands_for_update","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","cmp","command","command","command_for_update","eq","from","from","from","from_arg_matches","from_arg_matches","from_arg_matches_mut","from_arg_matches_mut","group_id","has_subcommand","into","into","into","into","log_level_filter","partial_cmp","to_owned","to_possible_value","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches_mut","update_from_arg_matches_mut","value_variants","vzip","vzip","vzip","dry_run","file","skip_chsh","Brew","Brew","Create","Create","Exec","Exec","KnownAction","KnownDirective","Link","Link","Subconfig","Subconfig","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","data","execute","from","from","from","from","from","from","from","into","into","parse_action_list","parse_context_defaults","to_owned","try_from","try_from","try_from","try_into","try_into","type_id","type_id","vzip","vzip","process"],"q":["dotfiles_processor","","","","dotfiles_processor::context","","","","","","","","","","","","","","","","","","dotfiles_processor::flags","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","dotfiles_processor::flags::Command","","","dotfiles_processor::known_directive","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","dotfiles_processor::process"],"d":["","","","","A context represents an environment in which defaults can …","The list of actions parsed from this file.","","","The default overrides for the current context.","The absolute path to the file to which this context …","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","Runs the actions in this context and consumes the context.","","","","","","","","Applies the configuration in the file passed as argument.","","Corresponds to the <code>Debug</code> log level.","Corresponds to the <code>Error</code> log level.","","Corresponds to the <code>Info</code> log level.","Installs Homebrew on Mac or Linuxbrew on Linux, see …","Installs Oh My Zsh! and sets <code>zsh</code> as the shell, see …","","A level lower than all log levels.","Corresponds to the <code>Trace</code> log level.","Corresponds to the <code>Warn</code> log level.","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","Only parse the configuration, do not run it.","Configuration file that describes what to do.","Skips running <code>chsh</code> to set <code>zsh</code> as the shell","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","",""],"i":[0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,19,0,13,13,0,13,19,19,0,13,13,13,17,17,19,19,17,13,19,17,13,19,13,13,13,17,17,17,13,17,13,19,17,19,17,19,17,19,17,13,13,19,17,13,13,13,17,13,19,17,13,19,17,13,19,17,19,17,19,13,17,13,19,32,32,33,25,23,25,23,25,23,0,0,25,23,25,23,25,23,25,23,23,23,23,25,25,25,25,25,25,25,23,25,23,23,23,23,25,23,23,25,23,25,23,25,23,0],"f":[0,0,0,0,0,[1,2],[[]],[[]],[1,3],[1,4],[[]],[[1,5,5],[[7,[6]]]],[[]],[1,[[9,[8]]]],[1,[[9,[8]]]],[[1,10],[[9,[1,8]]]],[10,[[9,[1]]]],[[],9],[5,[[9,[1]]]],[[],9],[[],11],[[]],0,0,0,0,0,0,0,0,0,0,0,0,[12,12],[12,12],[12,12],[12,12],[[]],[[]],[[]],[[]],[[]],[[]],[13,13],[[]],[[13,13],14],[[],12],0,[[],12],[[13,13],15],[[]],[[]],[[]],[16,[[9,[17,18]]]],[16,[[9,[19,18]]]],[16,[[9,[17,18]]]],[16,[[9,[19,18]]]],[[],[[7,[20]]]],[5,15],[[]],[[]],[13,21],[[]],0,[[13,13],[[7,[14]]]],[[]],[13,[[7,[22]]]],[[],9],[[],9],[[],9],[[],9],[[],9],[[],9],[[],11],[[],11],[[],11],[[17,16],[[9,[18]]]],[[19,16],[[9,[18]]]],[[17,16],[[9,[18]]]],[[19,16],[[9,[18]]]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[23,23],[[]],[23,24],[25,[[9,[8]]]],[26,25],[27,25],[28,25],[1,25],[29,25],[[]],[[]],[[]],[[]],[[23,30,31,1],[[9,[[2,[25]],8]]]],[[23,31],[[9,[8]]]],[[]],[[],9],[5,[[9,[23]]]],[[],9],[[],9],[[],9],[[],11],[[],11],[[]],[[]],[17,[[9,[8]]]]],"p":[[3,"Context"],[3,"Vec"],[3,"HashMap"],[3,"PathBuf"],[15,"str"],[4,"Setting"],[4,"Option"],[3,"DotfilesError"],[4,"Result"],[3,"Path"],[3,"TypeId"],[3,"Command"],[4,"LogLevelFilter"],[4,"Ordering"],[15,"bool"],[3,"ArgMatches"],[3,"FlagData"],[6,"Error"],[4,"Command"],[3,"Id"],[4,"LevelFilter"],[3,"PossibleValue"],[4,"KnownDirective"],[3,"DirectiveData"],[4,"KnownAction"],[3,"ExecAction"],[3,"BrewAction"],[6,"NativeLinkAction"],[6,"NativeCreateAction"],[6,"Settings"],[4,"StrictYaml"],[13,"ApplyConfig"],[13,"InstallOhMyZsh"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
