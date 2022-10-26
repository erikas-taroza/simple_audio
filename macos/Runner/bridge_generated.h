#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_Player {
  bool is_playing;
} wire_Player;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

void wire_new__static_method__Player(int64_t port_);

void wire_playback_state_stream__static_method__Player(int64_t port_);

void wire_get_is_playing__method__Player(int64_t port_, struct wire_Player *that);

void wire_open__method__Player(int64_t port_,
                               struct wire_Player *that,
                               struct wire_uint_8_list *path);

void wire_play__method__Player(int64_t port_, struct wire_Player *that);

void wire_pause__method__Player(int64_t port_, struct wire_Player *that);

struct wire_Player *new_box_autoadd_player_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_new__static_method__Player);
    dummy_var ^= ((int64_t) (void*) wire_playback_state_stream__static_method__Player);
    dummy_var ^= ((int64_t) (void*) wire_get_is_playing__method__Player);
    dummy_var ^= ((int64_t) (void*) wire_open__method__Player);
    dummy_var ^= ((int64_t) (void*) wire_play__method__Player);
    dummy_var ^= ((int64_t) (void*) wire_pause__method__Player);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_player_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}