import 'package:shell/wayland/model/event/meta_window_patches/meta_window_patches.serializable.dart';
import 'package:shell/wayland/model/request/wayland_request.dart';

/// [MetaWindowPatchesRequest]
class MetaWindowPatchesRequest extends WaylandRequest {
  /// constructor
  const MetaWindowPatchesRequest({
    required MetaWindowPatchMessage super.message,
    super.method = 'meta_window_patches',
  });
}
