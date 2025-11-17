// NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
// Copyright (C) 2024-2025 Chernov Denys

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// System metrics visualization components
// Упрощенная версия без Canvas (iced 0.12 не имеет встроенного canvas)
use iced::{widget::{container, text, column, row, progress_bar}, Element, Color, Length};

use crate::app::Message;
use crate::theme::{CyberColors, text_size, spacing};

// Метрики системы
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub cpu_usage: f32,      // 0.0 - 1.0
    pub memory_usage: f32,   // 0.0 - 1.0
    pub temperature: f32,    // °C
    pub disk_io: f32,        // 0.0 - 1.0
    pub network: f32,        // 0.0 - 1.0
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.42,
            memory_usage: 0.65,
            temperature: 58.0,
            disk_io: 0.25,
            network: 0.35,
        }
    }
}

// ============================================================================
// CPU Visualizer (упрощенный с progress bar)
// ============================================================================
fn cpu_color(usage: f32) -> Color {
    match usage {
        x if x < 0.3 => CyberColors::ACCENT_PRIMARY,
        x if x < 0.6 => CyberColors::ACCENT_BLUE,
        x if x < 0.85 => CyberColors::ACCENT_PURPLE,
        _ => CyberColors::STATUS_CRITICAL,
    }
}

fn metric_view<'a>(label: &str, value: f32, color: Color, unit: &str) -> Element<'a, Message> {
    container(
        column![
            text(label)
                .size(text_size::SM)
                .style(iced::theme::Text::Color(CyberColors::TEXT_SECONDARY)),
            progress_bar(0.0..=1.0, value)
                .height(Length::Fixed(8.0))
                .width(Length::Fixed(200.0)),
            text(format!("{}{}", (value * 100.0) as u32, unit))
                .size(text_size::LG)
                .style(iced::theme::Text::Color(color)),
        ]
        .spacing(spacing::SM as f32)
        .align_items(iced::Alignment::Center)
    )
    .padding(spacing::BASE)
    .style(iced::theme::Container::Custom(Box::new(MetricCardStyle)))
    .into()
}

struct MetricCardStyle;

impl iced::widget::container::StyleSheet for MetricCardStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(CyberColors::BG_SECONDARY)),
            border: iced::Border {
                color: CyberColors::BG_HOVER,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        }
    }
}

fn temp_color(temp: f32) -> Color {
    match temp {
        t if t < 40.0 => Color::from_rgb(0.0, 1.0, 1.0),
        t if t < 65.0 => CyberColors::ACCENT_BLUE,
        t if t < 80.0 => CyberColors::ACCENT_PURPLE,
        t if t < 90.0 => CyberColors::STATUS_WARNING,
        _ => CyberColors::STATUS_CRITICAL,
    }
}

// ============================================================================
// Metrics Dashboard View
// ============================================================================
pub struct MetricsDashboard {
    metrics: SystemMetrics,
}

impl MetricsDashboard {
    pub fn new() -> Self {
        Self {
            metrics: SystemMetrics::default(),
        }
    }

    pub fn update_metrics(&mut self, metrics: SystemMetrics) {
        self.metrics = metrics;
    }

    pub fn view(&self) -> Element<Message> {
        container(
            column![
                text("System Metrics")
                    .size(text_size::XXL)
                    .style(iced::theme::Text::Color(CyberColors::TEXT_PRIMARY)),
                row![
                    metric_view("CPU Load", self.metrics.cpu_usage, cpu_color(self.metrics.cpu_usage), "%"),
                    metric_view("Memory", self.metrics.memory_usage, CyberColors::ACCENT_BLUE, "%"),
                    metric_view("Disk I/O", self.metrics.disk_io, CyberColors::ACCENT_PURPLE, "%"),
                ]
                .spacing(spacing::LG as f32),
                row![
                    metric_view("Network", self.metrics.network, CyberColors::ACCENT_PRIMARY, "%"),
                    container(
                        column![
                            text("Temperature")
                                .size(text_size::SM)
                                .style(iced::theme::Text::Color(CyberColors::TEXT_SECONDARY)),
                            text(format!("{}°C", self.metrics.temperature as u32))
                                .size(text_size::DISPLAY)
                                .style(iced::theme::Text::Color(temp_color(self.metrics.temperature))),
                        ]
                        .spacing(spacing::SM as f32)
                        .align_items(iced::Alignment::Center)
                    )
                    .padding(spacing::BASE)
                    .style(iced::theme::Container::Custom(Box::new(MetricCardStyle))),
                ]
                .spacing(spacing::LG as f32),
            ]
            .spacing(spacing::XL as f32)
        )
        .padding(spacing::XL)
        .into()
    }
}
