[🪐 ARCANUM](https://arcanum.cosmere.cloud)
================================

<p align="center">
	<a href="https://github.com/didacdomenech/arcanum/stargazers">
		<img alt="Stargazers" src="https://img.shields.io/github/stars/didacdomenech/arcanum?style=for-the-badge&logo=starship&color=C9CBFF&logoColor=D9E0EE&labelColor=302D41"></a>
	<a href="https://github.com/didacdomenech/arcanum/issues">
		<img alt="Issues" src="https://img.shields.io/github/issues/didacdomenech/arcanum?style=for-the-badge&logo=gitbook&color=B5E8E0&logoColor=D9E0EE&labelColor=302D41"></a>
</p>


> [!IMPORTANT]
> Please read the original [README.md](https://github.com/Huxpro/huxpro.github.io) first to have the full guide.

This is a [fork](https://github.com/Huxpro/huxpro.github.io) of a [fork](https://github.com/HynDuf/hynduf.github.io) of the incredible [Hux Blog - Jekyll Theme].

> [!NOTE]
> I plan to refactor and change a lot of this in the future but I set this fork as a foundation for my personal blog. If you want to use this, I recommend checking the original author of this code mentioned above.

## Customize
The files you typically change are in the directories `_includes`, `_layouts`, and `less`. Most of the new changes I made are in `less/hynduf.less`.

Summarizing the changes I made:
- Home page view (the banner image took up so much space, so I replaced it with some random quotes of my own).
- Added tags of posts on the home page and improved the tags' appearance.
- Changed fonts (see in `variables.less`), increased font size, and reordered many things to my liking.
- Updated Font Awesome v5 (there might be some icons that I didn't use and haven't been migrated to the new version, please create an issue if you find any).
- Github action `jekyll.yml` file that automatically build and deploy the site after committing. You might need to change it if you fork this.
- [Catppuccin color palette](https://github.com/catppuccin/catppuccin)
    - Latte theme and Rosewater accent color for light mode.
    - Mocha theme and Mauve accent color for dark mode.
- Dark mode switcher.

TODOS:
- [ ] **Streamline Multilanguage Option**: Current workflow requires user to change too many things.
- [ ] **Projects** page with grid-like and image preview for projects.
- [ ] **Documentation-like Posts**: It's like a group-of-posts feature (Ex: Mkdocs, Just the docs...). Each post will have a left bar to navigate to other posts in the same group.

## Preview
Here are some previews of the revamped website.

![home-light](assets/home-light.png)
![home-dark](assets/home-dark.png)
![about-light](assets/about-light.png)
![about-dark](assets/about-dark.png)
![archive-light](assets/archive-light.png)
![archive-dark](assets/archive-dark.png)
![post-light](assets/post-light.png)
![post-dark](assets/post-dark.png)