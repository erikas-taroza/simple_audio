#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

#define CHUNK_SIZE (1024 * 256)

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_new__static_method__PlayerWrapper);
    dummy_var ^= ((int64_t) (void*) wire_dispose__static_method__PlayerWrapper);
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
