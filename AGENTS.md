# Account Analyzer - AI Agent Instructions

## Overview
This is a Rust-based application for analyzing bank statements and extracting data based on user queries. It provides a desktop application interface using Dioxus with web capabilities.

You are an expert Rust software engineer. Your task is to write highly idiomatic, safe, and performant Rust code (Edition 2024). 
Always adhere to the following rules:
1. Prioritize safe Rust; avoid 'unsafe' blocks unless explicitly requested.
2. Use proper error handling with Result and Option. Avoid excessive use of .unwrap().
3. Leverage Rust's type system, traits, and generics effectively.
4. Follow standard naming conventions (snake_case for functions/variables, PascalCase for types/traits).
5. Ensure strict ownership and borrowing rules are respected. Provide comments explaining complex lifetimes if necessary.

## Tech Stack

### Core Language
- **Rust 1.95.0** - Memory-safe systems programming language

### Frameworks and Libraries
- **Dioxus 0.7.10** - Cross-platform UI framework with router support
- **calamine 0.35.0** - Excel file parsing library with chrono support
- **anyhow 1.0.104** - error handling library
- **chrono 0.4.45** - Date and time handling library with serde support
- **dioxus-primitives** - Dioxus component primitives (from GitHub repo)
- **dioxus-tabular** - Table rendering components for Dioxus

### Build Features
- **default** - Enables desktop feature
- **web** - Enables web build target features (requires dioxus/web)
- **desktop** - Enables desktop build target features (requires dioxus/desktop)
- **mobile** - Enables mobile build target features (requires dioxus/mobile)

## Application Purpose
The application analyzes bank statements (likely Excel files) and allows users to query specific data points from their financial records.

## Key Capabilities
1. Parse and analyze bank statement files
2. Extract structured financial data
3. Provide query-based data retrieval
4. Display

## Styling Guidelines

### Use Tailwind CSS First
Always prefer using **Tailwind CSS utility classes** for styling components over writing custom CSS. The project includes `tailwind.css` with the Tailwind import already configured (v4.1.5).

1. **Prefer inline Tailwind classes in Dioxus rsx!** - Apply Tailwind utility classes directly to elements using the `class` attribute:
   ```rust
   div { class: "flex flex-col items-center p-4 bg-gray-900 rounded-lg", }
   ```

2. **Use only when necessary** - Write custom CSS in dedicated `.css` files (e.g., `assets/styling/`) only when:
   - Tailwind utility classes cannot achieve the desired layout or effect
   - You need complex pseudo-element styling (`::before`, `::after`)
   - You require advanced animations or transitions not easily expressed with Tailwind utilities
   - Building reusable component styles that are used across multiple components

3. **Custom CSS best practices**:
   - Keep custom CSS scoped to specific class names (avoid global element selectors)
   - Follow the existing dark theme color palette defined in `assets/dx-components-theme.css`
   - Use CSS variables where possible for theming consistency
   - Include responsive breakpoints using `@media` queries

4. **Asset loading** - Custom CSS files must be loaded via `asset!()` macro in `main.rs` and referenced with `document::Link` if needed per-component, though global stylesheets are preferred.

5. **Current Tailwind setup note**: The project has Tailwind CSS v4 installed but the generated utility classes are minimal (only a few utilities like `.table`, `.visible`, etc.). When Tailwind is fully configured with all utilities available, prefer using those over custom CSS. For now, continue writing scoped custom CSS for complex layouts and component styles.

## Bank Statement Format Examples

### ICICI Bank Statement Format
The application supports ICICI bank statements with the following column structure:

| Column Name | Description |
|-------------|-------------|
| S No. | Serial number of the transaction |
| Value Date | Date when the transaction is posted to the account |
| Transaction Date | Date when the transaction occurred |

| Transaction Remarks | Description or remarks about the transaction |
| Withdrawal Amount (INR) | Amount debited from the account in Indian Rupees |
| Deposit Amount (INR) | Amount credited to the account in Indian Rupees |
| Balance (INR) | Account balance after the transaction in Indian Rupees |

Example data row:
| 1	| 05/04/2025 | 05/04/2025 | ACH/RACPC SHANKARSHETH R/ICIC0000000004713089/NODN | 24973.00 | 0.0 | 35537.61
| 2 | 05/04/2025 | 05/04/2025 | ACH/RACPC - II WAKDEWADI/ICIC0000000007630007/NODN | 14934.00 | 0.0 | 20603.61

