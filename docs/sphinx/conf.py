import sphinx_rtd_theme
from datetime import datetime
from pygments.lexers.gdscript import GDScriptLexer
from pygments.token import Name
from sphinx.highlighting import lexers

# -- Project information -----------------------------------------------------


def get_copyright_years(first_year):
    current_year = datetime.now().year
    if current_year <= first_year:
        return str(first_year)

    return f"{first_year}-{current_year}"


project = "NaughtyAttributesGD"
copyright = f"{get_copyright_years(2026)} Denis Rizov"
author = "Denis Rizov"
release = "0.1.0"

# -- General configuration ---------------------------------------------------

extensions = [
    "sphinx_rtd_theme"
]

highlight_language = "gdscript"


# Pygments' GDScript lexer knows no annotations, so every "@export" comes out as an
# error token in a red box. Teach it the one rule it is missing.
class AnnotatedGDScriptLexer(GDScriptLexer):
    tokens = dict(GDScriptLexer.tokens)
    tokens["root"] = [(r"@[A-Za-z_]\w*", Name.Decorator)] + tokens["root"]


lexers["gdscript"] = AnnotatedGDScriptLexer()

templates_path = ["templates"]
exclude_patterns = []

# -- HTML output -------------------------------------------------------------

# Without this the browser title is built from project and release.
html_title = "NaughtyAttributes for Godot"

# Drops the _sources\ folder and the "View page source" link that points at it.
html_copy_source = False

# Replace that link with one that points at the page's source on GitHub.
html_context = {
    "site_title": "NaughtyAttributes’ Docs for Godot",
    "display_github": True,
    "github_user": "dbrizov",
    "github_repo": "NaughtyAttributesGD",
    "github_version": "master",
    "conf_py_path": "/docs/sphinx/src/",  # path to the .rst files, from the repo root
}

html_theme = "sphinx_rtd_theme"
html_theme_options = {
    "collapse_navigation": False,  # False makes the navigation tree-like
}

html_static_path = ["static"]
html_css_files = ["css/custom.css"]
