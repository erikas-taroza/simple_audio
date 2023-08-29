#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

#define CHUNK_SIZE (1024 * 256)

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct SimpleAudio_wire_list_media_control_action {
  int32_t *ptr;
  int32_t len;
} SimpleAudio_wire_list_media_control_action;

typedef struct SimpleAudio_wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} SimpleAudio_wire_uint_8_list;

typedef struct SimpleAudio_wire_Controls {
  const void *ptr;
} SimpleAudio_wire_Controls;

typedef struct SimpleAudio_wire_Player {
  struct SimpleAudio_wire_Controls controls;
} SimpleAudio_wire_Player;

typedef struct SimpleAudio_wire_Metadata {
  struct SimpleAudio_wire_uint_8_list *title;
  struct SimpleAudio_wire_uint_8_list *artist;
  struct SimpleAudio_wire_uint_8_list *album;
  struct SimpleAudio_wire_uint_8_list *art_uri;
  struct SimpleAudio_wire_uint_8_list *art_bytes;
} SimpleAudio_wire_Metadata;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void SimpleAudio_wire_new__static_method__Player(int64_t port_,
                                                 struct SimpleAudio_wire_list_media_control_action *actions,
                                                 struct SimpleAudio_wire_uint_8_list *dbus_name,
                                                 int64_t *hwnd);

void SimpleAudio_wire_dispose__static_method__Player(int64_t port_);

void SimpleAudio_wire_playback_state_stream__static_method__Player(int64_t port_);

void SimpleAudio_wire_progress_state_stream__static_method__Player(int64_t port_);

void SimpleAudio_wire_callback_stream__static_method__Player(int64_t port_);

void SimpleAudio_wire_is_playing__method__Player(int64_t port_,
                                                 struct SimpleAudio_wire_Player *that);

void SimpleAudio_wire_has_preloaded__method__Player(int64_t port_,
                                                    struct SimpleAudio_wire_Player *that);

void SimpleAudio_wire_get_progress__method__Player(int64_t port_,
                                                   struct SimpleAudio_wire_Player *that);

void SimpleAudio_wire_open__method__Player(int64_t port_,
                                           struct SimpleAudio_wire_Player *that,
                                           struct SimpleAudio_wire_uint_8_list *path,
                                           bool autoplay);

void SimpleAudio_wire_preload__method__Player(int64_t port_,
                                              struct SimpleAudio_wire_Player *that,
                                              struct SimpleAudio_wire_uint_8_list *path);

void SimpleAudio_wire_play_preload__method__Player(int64_t port_,
                                                   struct SimpleAudio_wire_Player *that);

void SimpleAudio_wire_play__method__Player(int64_t port_, struct SimpleAudio_wire_Player *that);

void SimpleAudio_wire_pause__method__Player(int64_t port_, struct SimpleAudio_wire_Player *that);

void SimpleAudio_wire_stop__method__Player(int64_t port_, struct SimpleAudio_wire_Player *that);

void SimpleAudio_wire_loop_playback__method__Player(int64_t port_,
                                                    struct SimpleAudio_wire_Player *that,
                                                    bool should_loop);

void SimpleAudio_wire_set_volume__method__Player(int64_t port_,
                                                 struct SimpleAudio_wire_Player *that,
                                                 float volume);

void SimpleAudio_wire_seek__method__Player(int64_t port_,
                                           struct SimpleAudio_wire_Player *that,
                                           uint64_t seconds);

void SimpleAudio_wire_set_metadata__method__Player(int64_t port_,
                                                   struct SimpleAudio_wire_Player *that,
                                                   struct SimpleAudio_wire_Metadata *metadata);

void SimpleAudio_wire_normalize_volume__method__Player(int64_t port_,
                                                       struct SimpleAudio_wire_Player *that,
                                                       bool should_normalize);

struct SimpleAudio_wire_Controls new_Controls(void);

int64_t *new_box_autoadd_i64_0(int64_t value);

struct SimpleAudio_wire_Metadata *new_box_autoadd_metadata_0(void);

struct SimpleAudio_wire_Player *new_box_autoadd_player_0(void);

struct SimpleAudio_wire_list_media_control_action *new_list_media_control_action_0(int32_t len);

struct SimpleAudio_wire_uint_8_list *new_uint_8_list_0(int32_t len);

void drop_opaque_Controls(const void *ptr);

const void *share_opaque_Controls(const void *ptr);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t SimpleAudio_dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_new__static_method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_dispose__static_method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_playback_state_stream__static_method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_progress_state_stream__static_method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_callback_stream__static_method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_is_playing__method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_has_preloaded__method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_get_progress__method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_open__method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_preload__method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_play_preload__method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_play__method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_pause__method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_stop__method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_loop_playback__method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_set_volume__method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_seek__method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_set_metadata__method__Player);
    dummy_var ^= ((int64_t) (void*) SimpleAudio_wire_normalize_volume__method__Player);
    dummy_var ^= ((int64_t) (void*) new_Controls);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i64_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_metadata_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_player_0);
    dummy_var ^= ((int64_t) (void*) new_list_media_control_action_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) drop_opaque_Controls);
    dummy_var ^= ((int64_t) (void*) share_opaque_Controls);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
