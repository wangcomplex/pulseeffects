/*
 *  Copyright © 2017-2020 Wellington Wallace
 *
 *  This file is part of PulseEffects.
 *
 *  PulseEffects is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  PulseEffects is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with PulseEffects.  If not, see <https://www.gnu.org/licenses/>.
 */

#include "deesser.hpp"
#include <glibmm/main.h>
#include "util.hpp"

namespace {

void on_post_messages_changed(GSettings* settings, gchar* key, Deesser* l) {
  const auto post = g_settings_get_boolean(settings, key);

  if (post) {
    if (!l->compression_connection.connected()) {
      l->compression_connection = Glib::signal_timeout().connect(
          [l]() {
            float compression = 0.0F;

            g_object_get(l->deesser, "compression", &compression, nullptr);

            l->compression.emit(compression);

            return true;
          },
          100);
    }

    if (!l->detected_connection.connected()) {
      l->detected_connection = Glib::signal_timeout().connect(
          [l]() {
            float detected = 0.0F;

            g_object_get(l->deesser, "detected", &detected, nullptr);

            l->detected.emit(detected);

            return true;
          },
          100);
    }
  } else {
    l->compression_connection.disconnect();
    l->detected_connection.disconnect();
  }
}

}  // namespace

Deesser::Deesser(const std::string& tag, const std::string& schema, const std::string& schema_path)
    : PluginBase(tag, "deesser", schema, schema_path) {
  deesser = gst_element_factory_make("calf-sourceforge-net-plugins-Deesser", nullptr);

  if (is_installed(deesser)) {
    auto* in_level = gst_element_factory_make("level", "deesser_input_level");
    auto* out_level = gst_element_factory_make("level", "deesser_output_level");
    auto* audioconvert_in = gst_element_factory_make("audioconvert", "deesser_audioconvert_in");
    auto* audioconvert_out = gst_element_factory_make("audioconvert", "deesser_audioconvert_out");

    gst_bin_add_many(GST_BIN(bin), in_level, audioconvert_in, deesser, audioconvert_out, out_level, nullptr);

    gst_element_link_many(in_level, audioconvert_in, deesser, audioconvert_out, out_level, nullptr);

    auto* pad_sink = gst_element_get_static_pad(in_level, "sink");
    auto* pad_src = gst_element_get_static_pad(out_level, "src");

    gst_element_add_pad(bin, gst_ghost_pad_new("sink", pad_sink));
    gst_element_add_pad(bin, gst_ghost_pad_new("src", pad_src));

    gst_object_unref(GST_OBJECT(pad_sink));
    gst_object_unref(GST_OBJECT(pad_src));

    g_object_set(deesser, "bypass", 0, nullptr);

    bind_to_gsettings();

    g_signal_connect(settings, "changed::post-messages", G_CALLBACK(on_post_messages_changed), this);

    g_settings_bind(settings, "post-messages", in_level, "post-messages", G_SETTINGS_BIND_DEFAULT);
    g_settings_bind(settings, "post-messages", out_level, "post-messages", G_SETTINGS_BIND_DEFAULT);

    if (g_settings_get_boolean(settings, "state") != 0) {
      enable();
    }
  }
}

Deesser::~Deesser() {
  util::debug(log_tag + name + " destroyed");
}

void Deesser::bind_to_gsettings() {
  g_settings_bind(settings, "detection", deesser, "detection", G_SETTINGS_BIND_DEFAULT);
  g_settings_bind(settings, "mode", deesser, "mode", G_SETTINGS_BIND_DEFAULT);

  g_settings_bind_with_mapping(settings, "ratio", deesser, "ratio", G_SETTINGS_BIND_GET, util::double_to_float, nullptr,
                               nullptr, nullptr);

  g_settings_bind_with_mapping(settings, "threshold", deesser, "threshold", G_SETTINGS_BIND_DEFAULT,
                               util::db20_gain_to_linear, util::linear_gain_to_db20, nullptr, nullptr);

  g_settings_bind(settings, "laxity", deesser, "laxity", G_SETTINGS_BIND_DEFAULT);

  g_settings_bind_with_mapping(settings, "makeup", deesser, "makeup", G_SETTINGS_BIND_DEFAULT,
                               util::db20_gain_to_linear, util::linear_gain_to_db20, nullptr, nullptr);

  g_settings_bind_with_mapping(settings, "f1-freq", deesser, "f1-freq", G_SETTINGS_BIND_GET, util::double_to_float,
                               nullptr, nullptr, nullptr);

  g_settings_bind_with_mapping(settings, "f2-freq", deesser, "f2-freq", G_SETTINGS_BIND_GET, util::double_to_float,
                               nullptr, nullptr, nullptr);

  g_settings_bind_with_mapping(settings, "f1-level", deesser, "f1-level", G_SETTINGS_BIND_DEFAULT,
                               util::db20_gain_to_linear, util::linear_gain_to_db20, nullptr, nullptr);

  g_settings_bind_with_mapping(settings, "f2-level", deesser, "f2-level", G_SETTINGS_BIND_DEFAULT,
                               util::db20_gain_to_linear, util::linear_gain_to_db20, nullptr, nullptr);

  g_settings_bind_with_mapping(settings, "f2-q", deesser, "f2-q", G_SETTINGS_BIND_GET, util::double_to_float, nullptr,
                               nullptr, nullptr);

  g_settings_bind(settings, "sc-listen", deesser, "sc-listen", G_SETTINGS_BIND_DEFAULT);
}
