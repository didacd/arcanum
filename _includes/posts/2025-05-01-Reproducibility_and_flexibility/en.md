Throughout my career, I’ve worked on a variety of projects—each unique in its approach, philosophy, and challenges. Whether it was adapting to different coding styles, development methodologies, or navigating the quirks of legacy code, the most difficult part was always adjusting to a new environment.

Over time, I explored various solutions to make that transition easier. I tried multiple IDEs, but working closely with servers pushed me (*forced* me) to get comfortable with the terminal. That’s when I discovered—and eventually came to love—**Vim**.

My learning curve with Vim was steep. Customizing the editor to suit my setup introduced me to the minimalist world of **Neovim** and its powerful plugin ecosystem. I became obsessed with creating the “perfect” environment—one that minimized friction and maximized efficiency. Unfortunately, this also meant I spent a lot of time experimenting with countless configurations and plugins.

That’s when I found [**LunarVim**](https://www.lunarvim.org/), a fantastic open-source project that offered a cleaner, more structured approach to Neovim configuration. Its simplicity and philosophy appealed to me immediately. However, as I began using it across different operating systems and updating plugins, I ran into dependency and compatibility issues. I needed something more **reproducible** and **portable**.

That search led me to [**Nix**](https://github.com/NixOS/nix), a powerful tool that solved all my configuration and environment reproducibility concerns. Thanks to the [**NVF**](https://github.com/NotAShelf/nvf) project, setting up my ideal Neovim environment with Nix became incredibly straightforward.

> 💡 To share what I learned, I created a repository that provides a simple, ready-to-use starting point for a complete Neovim development environment—easy to install, reproducible, and designed to work seamlessly across platforms.

👉 Check it out here:
<span style="display: inline-flex; align-items: center;">
  <img
    src="https://raw.githubusercontent.com/NotAShelf/nvf/main/.github/assets/nvf-logo-work.svg"
    width="15px"
    height="auto"
    style="margin-right: 8px;" />
  <a href="https://github.com/didacd/dnc">didacd/dnc</a>
</span>

Hope you like it ;)
