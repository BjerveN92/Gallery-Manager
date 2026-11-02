# Gallery Manager

Lightweight desktop app for image management via the Cloudinary API.  
Built to simplify uploading, deleting and sorting images linked to a website.

> **Disclaimer:** This app uses the [Cloudinary API](https://cloudinary.com/documentation) in accordance with their [Terms of Use](https://cloudinary.com/tos).

## Stack

- **Backend:** Tauri 2 (Rust)
- **Frontend:** SvelteKit (upcoming)
- **API:** Cloudinary Admin API + Upload API

## Backend — Tauri Commands

| Command          | Description                              |
| ---------------- | ---------------------------------------- |
| `list_images`    | Fetches images from a Cloudinary folder  |
| `upload_image`   | Uploads image with folder + Order metadata |
| `delete_image`   | Deletes image by `public_id`             |
| `update_order`   | Updates Order metadata on an image       |

## Project Structure (backend)

```
src-tauri/src/
├── main.rs
├── commands/        # Tauri commands (upload, delete, list, metadata)
└── cloudinary/      # Auth (signing/timestamp) + shared HTTP client
```

## Configuration

Credentials are stored in `.env` in the project root:

```env
CLOUDINARY_CLOUD_NAME=
CLOUDINARY_API_KEY=
CLOUDINARY_API_SECRET=
```

## Status

- [ ] Backend architecture
- [ ] Tauri commands implementation
- [ ] SvelteKit frontend
- [ ] Drag-and-drop sorting
