// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><li class="part-title">Docs</li><li class="chapter-item expanded "><a href="01-installation.html"><strong aria-hidden="true">1.</strong> Installation</a></li><li class="chapter-item expanded "><a href="02-command-line-interface.html"><strong aria-hidden="true">2.</strong> Command Line Interface</a></li><li class="chapter-item expanded "><a href="03-standard-json.html"><strong aria-hidden="true">3.</strong> Standard JSON</a></li><li class="chapter-item expanded "><a href="04-combined-json.html"><strong aria-hidden="true">4.</strong> Combined JSON</a></li><li class="chapter-item expanded "><a href="05-linker.html"><strong aria-hidden="true">5.</strong> Linker</a></li><li class="chapter-item expanded affix "><li class="part-title">EraVM</li><li class="chapter-item expanded "><a href="eravm/00-glossary.html"><strong aria-hidden="true">6.</strong> Glossary</a></li><li class="chapter-item expanded "><a href="eravm/01-code-separation.html"><strong aria-hidden="true">7.</strong> Code Separation</a></li><li class="chapter-item expanded "><a href="eravm/02-evm-assembly-translator.html"><strong aria-hidden="true">8.</strong> EVM Assembly Translator</a></li><li class="chapter-item expanded "><a href="eravm/03-system-contracts.html"><strong aria-hidden="true">9.</strong> System Contracts</a></li><li class="chapter-item expanded "><a href="eravm/04-exception-handling.html"><strong aria-hidden="true">10.</strong> Exception Handling</a></li><li class="chapter-item expanded "><a href="eravm/05-instructions/01-reference.html"><strong aria-hidden="true">11.</strong> Instructions</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="eravm/05-instructions/02-evm/01-reference.html"><strong aria-hidden="true">11.1.</strong> Native EVM Instructions</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="eravm/05-instructions/02-evm/02-arithmetic.html"><strong aria-hidden="true">11.1.1.</strong> Arithmetic</a></li><li class="chapter-item expanded "><a href="eravm/05-instructions/02-evm/03-bitwise.html"><strong aria-hidden="true">11.1.2.</strong> Bitwise</a></li><li class="chapter-item expanded "><a href="eravm/05-instructions/02-evm/04-block.html"><strong aria-hidden="true">11.1.3.</strong> Block</a></li><li class="chapter-item expanded "><a href="eravm/05-instructions/02-evm/05-calls.html"><strong aria-hidden="true">11.1.4.</strong> Calls</a></li><li class="chapter-item expanded "><a href="eravm/05-instructions/02-evm/06-create.html"><strong aria-hidden="true">11.1.5.</strong> CREATE</a></li><li class="chapter-item expanded "><a href="eravm/05-instructions/02-evm/07-environment.html"><strong aria-hidden="true">11.1.6.</strong> Environment</a></li><li class="chapter-item expanded "><a href="eravm/05-instructions/02-evm/08-logging.html"><strong aria-hidden="true">11.1.7.</strong> Logging</a></li><li class="chapter-item expanded "><a href="eravm/05-instructions/02-evm/09-logical.html"><strong aria-hidden="true">11.1.8.</strong> Logical</a></li><li class="chapter-item expanded "><a href="eravm/05-instructions/02-evm/10-memory.html"><strong aria-hidden="true">11.1.9.</strong> Memory</a></li><li class="chapter-item expanded "><a href="eravm/05-instructions/02-evm/11-return.html"><strong aria-hidden="true">11.1.10.</strong> Return</a></li><li class="chapter-item expanded "><a href="eravm/05-instructions/02-evm/12-sha3.html"><strong aria-hidden="true">11.1.11.</strong> SHA3</a></li><li class="chapter-item expanded "><a href="eravm/05-instructions/02-evm/13-stack.html"><strong aria-hidden="true">11.1.12.</strong> Stack</a></li></ol></li><li class="chapter-item expanded "><a href="eravm/05-instructions/03-evm-assembly.html"><strong aria-hidden="true">11.2.</strong> EVM Assembly</a></li><li class="chapter-item expanded "><a href="eravm/05-instructions/04-yul.html"><strong aria-hidden="true">11.3.</strong> Yul</a></li></ol></li><li class="chapter-item expanded "><a href="eravm/06-extensions.html"><strong aria-hidden="true">12.</strong> Extensions</a></li><li class="chapter-item expanded "><a href="eravm/07-binary-layout.html"><strong aria-hidden="true">13.</strong> Binary Layout</a></li><li class="chapter-item expanded affix "><li class="part-title">Guides</li><li class="chapter-item expanded "><a href="guides/01-sanitizers.html"><strong aria-hidden="true">14.</strong> Building with sanitizers</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
