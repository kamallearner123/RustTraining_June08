use crate::parser::ValgrindReport;
use std::fs::File;
use std::io::Write;

pub fn generate_html(report: &ValgrindReport, output_path: &str) -> std::io::Result<()> {
    let mut html = String::new();

    // Tailwind setup and Dark Theme base
    html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Valgrind Report Analyzer</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <style>
        body { background-color: #0f172a; color: #f8fafc; font-family: 'Inter', sans-serif; }
        .glass { background: rgba(30, 41, 59, 0.7); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.1); }
        .scrollbar-hide::-webkit-scrollbar { display: none; }
        .scrollbar-hide { -ms-overflow-style: none; scrollbar-width: none; }
    </style>
</head>
<body class="min-h-screen p-8">
    <div class="max-w-6xl mx-auto space-y-8">
"#);

    // Header Dashboard
    html.push_str(&format!(r#"
        <header class="glass rounded-xl p-6 shadow-2xl flex flex-col md:flex-row items-center justify-between">
            <div>
                <h1 class="text-4xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-emerald-400">
                    Valgrind Report Analyzer
                </h1>
                <p class="text-slate-400 mt-2 font-mono">Command: <span class="text-emerald-300">{}</span> | PID: <span class="text-blue-300">{}</span></p>
            </div>
            <div class="mt-4 md:mt-0 text-right">
                <span class="inline-flex items-center gap-1.5 py-1.5 px-3 rounded-full text-xs font-medium bg-red-500/10 text-red-400 border border-red-500/20">
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z" clip-rule="evenodd"></path></svg>
                    {}
                </span>
            </div>
        </header>
"#, report.command, report.pid, report.error_summary));

    // Summary Cards
    html.push_str(r#"<div class="grid grid-cols-1 md:grid-cols-2 gap-6">"#);
    
    // Heap Summary Card
    html.push_str(r#"
        <div class="glass rounded-xl p-6 shadow-xl border-l-4 border-l-blue-500">
            <h2 class="text-xl font-bold text-slate-100 mb-4 flex items-center">
                <svg class="w-5 h-5 mr-2 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path></svg>
                Heap Summary
            </h2>
            <ul class="space-y-2 font-mono text-sm text-slate-300">
"#);
    if report.heap_summary.is_empty() {
        html.push_str("<li>No heap summary found.</li>");
    } else {
        for line in &report.heap_summary {
            html.push_str(&format!("<li>{}</li>", line));
        }
    }
    html.push_str("</ul></div>");

    // Leak Summary Card
    html.push_str(r#"
        <div class="glass rounded-xl p-6 shadow-xl border-l-4 border-l-red-500">
            <h2 class="text-xl font-bold text-slate-100 mb-4 flex items-center">
                <svg class="w-5 h-5 mr-2 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
                Leak Summary
            </h2>
            <ul class="space-y-2 font-mono text-sm text-slate-300">
"#);
    if report.leak_summary.is_empty() {
        html.push_str("<li>No leaks detected.</li>");
    } else {
        for line in &report.leak_summary {
            html.push_str(&format!("<li>{}</li>", line));
        }
    }
    html.push_str("</ul></div></div>");

    // Errors Section
    html.push_str(r#"
        <div>
            <h2 class="text-2xl font-bold mb-6 text-slate-100 border-b border-slate-700 pb-2">Detected Errors</h2>
            <div class="space-y-6">
"#);

    if report.errors.is_empty() {
        html.push_str(r#"
            <div class="glass rounded-xl p-8 text-center text-slate-400">
                <svg class="w-12 h-12 mx-auto text-emerald-500 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                <p class="text-lg">No memory errors found! Your code is pristine.</p>
            </div>
        "#);
    } else {
        for (i, err) in report.errors.iter().enumerate() {
            html.push_str(&format!(r#"
                <div class="glass rounded-xl p-0 overflow-hidden shadow-lg border border-red-900/50">
                    <div class="bg-red-500/20 px-6 py-4 border-b border-red-900/50 flex items-center justify-between">
                        <h3 class="text-lg font-semibold text-red-300 flex items-center gap-2">
                            <span class="bg-red-500/20 text-red-400 rounded-full w-6 h-6 flex items-center justify-center text-xs">{}</span>
                            {}
                        </h3>
                    </div>
                    <div class="p-6 bg-slate-900/50">
                        <h4 class="text-xs uppercase tracking-wider text-slate-500 mb-3 font-semibold">Stack Trace</h4>
                        <div class="space-y-2 font-mono text-sm overflow-x-auto scrollbar-hide">
            "#, i + 1, err.kind));

            for trace in &err.backtrace {
                // Highlight file paths or hex addresses slightly
                let styled_trace = trace
                    .replace(" at 0x", "<span class='text-purple-400'> at 0x</span>")
                    .replace(" by 0x", "<span class='text-purple-400'> by 0x</span>");

                html.push_str(&format!(r#"<div class="text-slate-300 bg-slate-800/50 p-2 rounded border border-slate-700/50">{}</div>"#, styled_trace));
            }

            html.push_str(r#"
                        </div>
                    </div>
                </div>
            "#);
        }
    }

    html.push_str(r#"
            </div>
        </div>
    </div>
</body>
</html>"#);

    let mut file = File::create(output_path)?;
    file.write_all(html.as_bytes())?;

    Ok(())
}
