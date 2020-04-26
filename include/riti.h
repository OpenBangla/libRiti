/* Text to put at the beginning of the generated file. Probably a license. */

#ifndef RITI_H
#define RITI_H

/* Generated with cbindgen:0.14.1 */

/* 
 * Warning, this file is autogenerated by cbindgen. Don't modify this manually.
 * Run this command to generate this file: cbindgen --config cbindgen.toml --output include/riti.h 
 */


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

/*
 Alt modifier key.

 Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
 */
static const uint8_t MODIFIER_ALT = (1 << 2);

/*
 Ctrl modifier key.

 Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
 */
static const uint8_t MODIFIER_CTRL = (1 << 1);

/*
 Shift modifier key.

 Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
 */
static const uint8_t MODIFIER_SHIFT = (1 << 0);

static const uint16_t VC_0 = 11;

static const uint16_t VC_1 = 2;

static const uint16_t VC_2 = 3;

static const uint16_t VC_3 = 4;

static const uint16_t VC_4 = 5;

static const uint16_t VC_5 = 6;

static const uint16_t VC_6 = 7;

static const uint16_t VC_7 = 8;

static const uint16_t VC_8 = 9;

static const uint16_t VC_9 = 10;

static const uint16_t VC_A = 41140;

static const uint16_t VC_ALT = 56;

static const uint16_t VC_AMPERSAND = 65;

static const uint16_t VC_APOSTROPHE = 40;

static const uint16_t VC_ASTERISK = 66;

static const uint16_t VC_AT = 60;

static const uint16_t VC_B = 41141;

static const uint16_t VC_BACKSPACE = 14;

static const uint16_t VC_BACK_SLASH = 43;

static const uint16_t VC_BAR = 93;

static const uint16_t VC_BRACE_LEFT = 91;

static const uint16_t VC_BRACE_RIGHT = 92;

static const uint16_t VC_BRACKET_LEFT = 26;

static const uint16_t VC_BRACKET_RIGHT = 27;

static const uint16_t VC_C = 41142;

static const uint16_t VC_CIRCUM = 64;

static const uint16_t VC_COLON = 99;

static const uint16_t VC_COMMA = 51;

static const uint16_t VC_CONTROL = 29;

static const uint16_t VC_D = 41143;

static const uint16_t VC_DELETE = 3667;

static const uint16_t VC_DOLLAR = 62;

static const uint16_t VC_DOWN = 57424;

static const uint16_t VC_E = 41144;

static const uint16_t VC_END = 3663;

static const uint16_t VC_ENTER = 28;

static const uint16_t VC_EQUALS = 13;

static const uint16_t VC_EXCLAIM = 59;

static const uint16_t VC_F = 41145;

static const uint16_t VC_G = 41146;

static const uint16_t VC_GRAVE = 41;

static const uint16_t VC_GREATER = 102;

static const uint16_t VC_H = 41147;

static const uint16_t VC_HASH = 61;

static const uint16_t VC_HOME = 3655;

static const uint16_t VC_I = 41148;

static const uint16_t VC_INSERT = 3666;

static const uint16_t VC_J = 41149;

static const uint16_t VC_K = 41150;

static const uint16_t VC_KP_0 = 82;

static const uint16_t VC_KP_1 = 79;

static const uint16_t VC_KP_2 = 80;

static const uint16_t VC_KP_3 = 81;

static const uint16_t VC_KP_4 = 75;

static const uint16_t VC_KP_5 = 76;

static const uint16_t VC_KP_6 = 77;

static const uint16_t VC_KP_7 = 71;

static const uint16_t VC_KP_8 = 72;

static const uint16_t VC_KP_9 = 73;

static const uint16_t VC_KP_ADD = 78;

static const uint16_t VC_KP_DECIMAL = 83;

static const uint16_t VC_KP_DIVIDE = 3637;

static const uint16_t VC_KP_ENTER = 3612;

static const uint16_t VC_KP_EQUALS = 3597;

static const uint16_t VC_KP_MULTIPLY = 55;

static const uint16_t VC_KP_SUBTRACT = 74;

static const uint16_t VC_L = 41151;

static const uint16_t VC_LEFT = 57419;

static const uint16_t VC_LESS = 101;

static const uint16_t VC_M = 41152;

static const uint16_t VC_MINUS = 12;

static const uint16_t VC_N = 41153;

static const uint16_t VC_O = 41154;

static const uint16_t VC_P = 41155;

static const uint16_t VC_PAGE_DOWN = 3665;

static const uint16_t VC_PAGE_UP = 3657;

static const uint16_t VC_PAREN_LEFT = 67;

static const uint16_t VC_PAREN_RIGHT = 68;

static const uint16_t VC_PERCENT = 63;

static const uint16_t VC_PERIOD = 52;

static const uint16_t VC_PLUS = 88;

static const uint16_t VC_Q = 41156;

static const uint16_t VC_QUESTION = 103;

static const uint16_t VC_QUOTE = 100;

static const uint16_t VC_R = 41157;

static const uint16_t VC_RIGHT = 57421;

static const uint16_t VC_S = 41158;

static const uint16_t VC_SEMICOLON = 39;

static const uint16_t VC_SHIFT = 42;

static const uint16_t VC_SLASH = 53;

static const uint16_t VC_SPACE = 57;

static const uint16_t VC_T = 41159;

static const uint16_t VC_TAB = 15;

static const uint16_t VC_TILDE = 1;

static const uint16_t VC_U = 41160;

static const uint16_t VC_UNDERSCORE = 87;

static const uint16_t VC_UNKNOWN = 70;

static const uint16_t VC_UP = 57416;

static const uint16_t VC_V = 41161;

static const uint16_t VC_W = 41162;

static const uint16_t VC_X = 41163;

static const uint16_t VC_Y = 41164;

static const uint16_t VC_Z = 41165;

static const uint16_t VC_a = 41110;

static const uint16_t VC_b = 41111;

static const uint16_t VC_c = 41112;

static const uint16_t VC_d = 41113;

static const uint16_t VC_e = 41114;

static const uint16_t VC_f = 41115;

static const uint16_t VC_g = 41116;

static const uint16_t VC_h = 41117;

static const uint16_t VC_i = 41118;

static const uint16_t VC_j = 41119;

static const uint16_t VC_k = 41120;

static const uint16_t VC_l = 41121;

static const uint16_t VC_m = 41122;

static const uint16_t VC_n = 41123;

static const uint16_t VC_o = 41124;

static const uint16_t VC_p = 41125;

static const uint16_t VC_q = 41126;

static const uint16_t VC_r = 41127;

static const uint16_t VC_s = 41128;

static const uint16_t VC_t = 41129;

static const uint16_t VC_u = 41130;

static const uint16_t VC_v = 41131;

static const uint16_t VC_w = 41132;

static const uint16_t VC_x = 41133;

static const uint16_t VC_y = 41134;

static const uint16_t VC_z = 41135;

/*
 Context handle used for libRiti IM APIs
 */
struct RitiContext;

/*
 Suggestions which are intend to be shown by the IM's candidate window.
 */
struct Suggestion;

extern "C" {

/*
 A candidate of the suggestion list was committed.

 `index`: index of the candidate.
 */
void riti_context_candidate_committed(RitiContext *ptr, uintptr_t index);

void riti_context_free(RitiContext *ptr);

/*
 Returns `true` if the key was handled, `false` otherwise.
 */
bool riti_context_key_handled(RitiContext *ptr);

RitiContext *riti_context_new();

/*
 Update the suggestion making engine. This would also look for changes
 in layout selection and AutoCorrect database.
 */
void riti_context_update_engine(RitiContext *ptr);

Suggestion *riti_get_suggestion_for_key(RitiContext *ptr,
                                        uint16_t key,
                                        uint8_t modifier);

void riti_suggestion_free(Suggestion *ptr);

char *riti_suggestion_get_auxiliary_text(const Suggestion *ptr);

uintptr_t riti_suggestion_get_length(const Suggestion *ptr);

/*
 Get the only suggestion of the *lonely* `Suggestion`.
 */
char *riti_suggestion_get_lonely_suggestion(const Suggestion *ptr);

char *const *riti_suggestion_get_suggestions(const Suggestion *ptr);

bool riti_suggestion_is_empty(const Suggestion *ptr);

/*
 Returns `true` when the `Suggestion` struct is a **lonely** one, otherwise returns `false`.

 A *lonely* `Suggestion` struct means that the struct has only one suggestion.
 */
bool riti_suggestion_is_lonely(const Suggestion *ptr);

/*
 Returns index of the suggestion, which was previously selected.
 */
uintptr_t riti_suggestion_previously_selected_index(const Suggestion *ptr);

} // extern "C"

#endif // RITI_H
