#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_Player {
  const void *ptr;
} wire_Player;

typedef struct wire_PlayerWrapper {
  struct wire_Player internal;
} wire_PlayerWrapper;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_new__static_method__PlayerWrapper(int64_t port_);

void wire_dispose__static_method__PlayerWrapper(int64_t port_);

void wire_playback_started_stream__static_method__PlayerWrapper(int64_t port_);

void wire_playback_state_stream__static_method__PlayerWrapper(int64_t port_);

void wire_progress_state_stream__static_method__PlayerWrapper(int64_t port_);

void wire_error_stream__static_method__PlayerWrapper(int64_t port_);

void wire_playback_state__method__PlayerWrapper(int64_t port_, struct wire_PlayerWrapper *that);

void wire_progress__method__PlayerWrapper(int64_t port_, struct wire_PlayerWrapper *that);

void wire_is_preloaded__method__PlayerWrapper(int64_t port_, struct wire_PlayerWrapper *that);

void wire_is_looping__method__PlayerWrapper(int64_t port_, struct wire_PlayerWrapper *that);

void wire_is_normalizing__method__PlayerWrapper(int64_t port_, struct wire_PlayerWrapper *that);

void wire_volume__method__PlayerWrapper(int64_t port_, struct wire_PlayerWrapper *that);

void wire_open__method__PlayerWrapper(int64_t port_,
                                      struct wire_PlayerWrapper *that,
                                      struct wire_uint_8_list *path,
                                      bool autoplay);

void wire_preload__method__PlayerWrapper(int64_t port_,
                                         struct wire_PlayerWrapper *that,
                                         struct wire_uint_8_list *path);

void wire_play_preload__method__PlayerWrapper(int64_t port_, struct wire_PlayerWrapper *that);

void wire_clear_preload__method__PlayerWrapper(int64_t port_, struct wire_PlayerWrapper *that);

void wire_play__method__PlayerWrapper(int64_t port_, struct wire_PlayerWrapper *that);

void wire_pause__method__PlayerWrapper(int64_t port_, struct wire_PlayerWrapper *that);

void wire_stop__method__PlayerWrapper(int64_t port_, struct wire_PlayerWrapper *that);

void wire_loop_playback__method__PlayerWrapper(int64_t port_,
                                               struct wire_PlayerWrapper *that,
                                               bool should_loop);

void wire_set_volume__method__PlayerWrapper(int64_t port_,
                                            struct wire_PlayerWrapper *that,
                                            float volume);

void wire_seek__method__PlayerWrapper(int64_t port_,
                                      struct wire_PlayerWrapper *that,
                                      int64_t position);

void wire_normalize_volume__method__PlayerWrapper(int64_t port_,
                                                  struct wire_PlayerWrapper *that,
                                                  bool should_normalize);

struct wire_Player new_Player(void);

struct wire_PlayerWrapper *new_box_autoadd_player_wrapper_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void drop_opaque_Player(const void *ptr);

const void *share_opaque_Player(const void *ptr);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_new__static_method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_dispose__static_method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_playback_started_stream__static_method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_playback_state_stream__static_method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_progress_state_stream__static_method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_error_stream__static_method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_playback_state__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_progress__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_is_preloaded__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_is_looping__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_is_normalizing__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_volume__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_open__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_preload__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_play_preload__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_clear_preload__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_play__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_pause__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_stop__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_loop_playback__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_set_volume__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_seek__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_normalize_volume__method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) new_Player);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_player_wrapper_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) drop_opaque_Player);
    dummy_var ^= ((int64_t) (void*) share_opaque_Player);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
