plugins {{
    kotlin("multiplatform")
    id("maven-publish")
}}

kotlin {{
    // Targets as in shared
}}

publishing {{
    publications {{
        maven(MavenPublication) {{
            from(components["kotlin"])
        }}
    }}
}}