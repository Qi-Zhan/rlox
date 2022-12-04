
mod forstmt {
    mod for_class_in_body;
    // mod for_closure_in_body;
    // mod for_fun_in_body;
    // mod for_return_closure;
    // mod for_return_inside;
    mod for_scope;
    mod for_statement_condition;
    mod for_statement_increment;
    mod for_statement_initializer;
    mod for_syntax; // function
    mod for_var_in_body;
}
mod whilestmt {
    mod while_syntax;
    mod while_class_in_body;
    mod while_closure_in_body;
    mod while_fun_in_body;
    mod while_return_closure;
    mod while_return_inside;
    mod while_var_in_body;
}
mod ifstmt {
    mod if_class_in_else;
    mod if_class_in_then;
    mod if_else;
    mod if_dangling_else;
    mod if_fun_in_else;
    mod if_fun_in_then;
    mod if_truth;
    mod if_var_in_else;
    mod if_var_in_then;

}

mod variable {
    mod variable_redeclare_global;
    mod variable_undefined_global;
    mod variable_use_nil_as_var;
    mod variable_use_this_as_var;
    mod variable_duplicate_local;
    mod variable_in_middle_of_block;
    mod variable_in_nested_block;
    mod variable_scope_reuse_in_different_blocks;
    mod variable_shadow_and_local;
    mod variable_shadow_global;
    mod variable_shadow_local;
    mod variable_undefined_local;
    mod variable_uninitialized;
    mod variable_use_false_as_var;
    mod variable_use_global_in_initializer;
    mod variable_collide_with_parameter;
    mod variable_duplicate_parameter;
    mod variable_early_bound;
    mod variable_local_from_method;
    mod variable_redefine_global;
    mod variable_unreached_undefined;
}

mod block {
    mod block_empty;
    mod block_scope;
}

mod primitive {
    mod print_missing_argument;
    mod precedence;
    mod bool_equality;
    mod bool_not;
    mod empty_file;
    mod expressions_evaluate;
    mod nil_literal;
    mod number_nan_equality;
    mod number_string_literals;
    mod unexpected_character;
}

mod assignment {
    mod assignment_associativity;
    mod assignment_global;
    mod assignment_local;
    mod assignment_grouping;
    mod assignment_infix_operator;
    mod assignment_prefix_operator;
    mod assignment_syntax;
    mod assignment_to_this;
    mod assignment_undefined;
}

mod operator {
    mod operator_add;
    mod operator_divide;
    mod operator_equals;
    mod operator_add_bool_nil;
    mod operator_add_bool_num;
    mod operator_add_bool_string;
    mod operator_add_nil_nil;
    mod operator_add_num_nil;
    mod operator_add_string_nil;
    mod operator_comparison;
    mod operator_divide_nonnum_num;
    mod operator_divide_num_nonnum;
    mod operator_equals_class;
    mod operator_equals_method;
    mod operator_greater_nonnum_num;
    mod operator_greater_num_nonnum;
    mod operator_greater_or_equal_nonnum_num;
    mod operator_greater_or_equal_num_nonnum;
    mod operator_less_nonnum_num;
    mod operator_less_num_nonnum;
    mod operator_less_or_equal_nonnum_num;
    mod operator_less_or_equal_num_nonnum;
    mod operator_multiply;
    mod operator_multiply_nonnum_num;
    mod operator_multiply_num_nonnum;
    mod operator_negate;
    mod operator_negate_nonnum;
    mod operator_not;
    mod operator_not_class;
    mod operator_not_equals;
    mod operator_subtract;
    mod operator_subtract_nonnum_num;
    mod operator_subtract_num_nonnum;
    mod logical_operator_and;
    mod logical_operator_and_truth;
    mod logical_operator_or;
    mod logical_operator_or_truth;

}
