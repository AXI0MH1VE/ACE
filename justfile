[just]
test = "pytest tests/"
lint = "black --check src/ tests/"
build = "uv build"
package = "uv build --sdist --wheel"
bench = "python bench/bench_core.py"
init = "uv sync"