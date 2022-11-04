#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_Player {
  int32_t dummy;
} wire_Player;

typedef struct wire_Metadata {
  struct wire_uint_8_list *title;
  struct wire_uint_8_list *artist;
  struct wire_uint_8_list *album;
  struct wire_uint_8_list *art_url;
} wire_Metadata;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

void wire_new__static_method__Player(int64_t port_,
                                     struct wire_uint_8_list *mpris_name,
                                     int64_t *hwnd);

void wire_playback_state_stream__static_method__Player(int64_t port_);

void wire_progress_state_stream__static_method__Player(int64_t port_);

void wire_metadata_callback_stream__static_method__Player(int64_t port_);

void wire_is_playing__method__Player(int64_t port_, struct wire_Player *that);

void wire_get_progress__method__Player(int64_t port_, struct wire_Player *that);

void wire_open__method__Player(int64_t port_,
                               struct wire_Player *that,
                               struct wire_uint_8_list *path,
                               bool autoplay);

void wire_play__method__Player(int64_t port_, struct wire_Player *that);

void wire_pause__method__Player(int64_t port_, struct wire_Player *that);

void wire_stop__method__Player(int64_t port_, struct wire_Player *that);

void wire_set_volume__method__Player(int64_t port_, struct wire_Player *that, float volume);

void wire_seek__method__Player(int64_t port_, struct wire_Player *that, uint64_t seconds);

void wire_set_metadata__method__Player(int64_t port_,
                                       struct wire_Player *that,
                                       struct wire_Metadata *metadata);

int64_t *new_box_autoadd_i64_0(int64_t value);

struct wire_Metadata *new_box_autoadd_metadata_0(void);

struct wire_Player *new_box_autoadd_player_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_new__static_method__Player);
    dummy_var ^= ((int64_t) (void*) wire_playback_state_stream__static_method__Player);
    dummy_var ^= ((int64_t) (void*) wire_progress_state_stream__static_method__Player);
    dummy_var ^= ((int64_t) (void*) wire_metadata_callback_stream__static_method__Player);
    dummy_var ^= ((int64_t) (void*) wire_is_playing__method__Player);
    dummy_var ^= ((int64_t) (void*) wire_get_progress__method__Player);
    dummy_var ^= ((int64_t) (void*) wire_open__method__Player);
    dummy_var ^= ((int64_t) (void*) wire_play__method__Player);
    dummy_var ^= ((int64_t) (void*) wire_pause__method__Player);
    dummy_var ^= ((int64_t) (void*) wire_stop__method__Player);
    dummy_var ^= ((int64_t) (void*) wire_set_volume__method__Player);
    dummy_var ^= ((int64_t) (void*) wire_seek__method__Player);
    dummy_var ^= ((int64_t) (void*) wire_set_metadata__method__Player);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i64_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_metadata_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_player_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}