<script lang="ts">
    import { FilterBuilder, FilterSummary, FilterTemplateManager } from "@orbis/ui/filter-builder";
    import type { UniversalFilter, FieldDefinition } from "@orbis/ui/filter-builder";
    import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@orbis/ui/card";
    import { Button } from "@orbis/ui/button";

    let filter: UniversalFilter = $state({
        logic: "AND",
        conditions: [],
        groups: []
    });

    const fields: FieldDefinition[] = [
        { id: "score", label: "Score", type: "number" },
        { id: "status", label: "Status", type: "select", options: [
            { label: "Active", value: "active" },
            { label: "Inactive", value: "inactive" },
            { label: "Pending", value: "pending" },
            { label: "Graduated", value: "graduated" }
        ]},
        { id: "enrollment_date", label: "Enrollment Date", type: "date" },
        { id: "is_active", label: "Is Active", type: "boolean" },
        { id: "name", label: "Student Name", type: "string" },
        { id: "email", label: "Email", type: "string" },
        { id: "grade_level", label: "Grade Level", type: "select", options: [
            { label: "Grade 1", value: "1" },
            { label: "Grade 2", value: "2" },
            { label: "Grade 3", value: "3" },
            { label: "Grade 4", value: "4" },
            { label: "Grade 5", value: "5" },
            { label: "Grade 6", value: "6" }
        ]},
        { id: "age", label: "Age", type: "number" },
        { id: "created_at", label: "Created At", type: "date" }
    ];

    function resetFilter() {
        filter = { logic: "AND", conditions: [], groups: [] };
    }
</script>

<div class="container mx-auto p-6 max-w-6xl">
    <div class="mb-8">
        <h1 class="text-3xl font-bold tracking-tight text-foreground">Filter Builder</h1>
        <p class="text-muted-foreground mt-2">Build complex, multi-condition filters visually. Supports AND/OR logic, nested groups, and template saving.</p>
    </div>

    <!-- Template Manager + Reset -->
    <div class="flex items-center justify-between mb-4">
        <FilterTemplateManager bind:filter />
        <Button variant="outline" size="sm" onclick={resetFilter}>Reset All</Button>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-[2fr_1fr] gap-6">
        <!-- Filter Builder Section -->
        <Card class="border-border/50 shadow-sm bg-card/50 backdrop-blur">
            <CardHeader>
                <CardTitle>Construct Filter</CardTitle>
                <CardDescription>Add rules and groups to build a complex query.</CardDescription>
            </CardHeader>
            <CardContent>
                <FilterBuilder bind:filter {fields} />
            </CardContent>
        </Card>

        <!-- Sidebar: Summary + JSON -->
        <div class="flex flex-col gap-6">
            <!-- Summary Card -->
            <Card class="border-border/50 shadow-sm bg-card/50 backdrop-blur">
                <CardHeader>
                    <CardTitle>Summary</CardTitle>
                    <CardDescription>Human-readable filter summary.</CardDescription>
                </CardHeader>
                <CardContent>
                    <FilterSummary {filter} {fields} />
                </CardContent>
            </Card>

            <!-- JSON Output Card -->
            <Card class="border-border/50 shadow-sm bg-card/50 backdrop-blur h-fit sticky top-6">
                <CardHeader>
                    <CardTitle>Generated Payload</CardTitle>
                    <CardDescription>This JSON will be sent to the backend.</CardDescription>
                </CardHeader>
                <CardContent>
                    <div class="h-[400px] w-full rounded-md border bg-muted/30 p-4 overflow-y-auto">
                        <pre class="text-xs font-mono text-muted-foreground whitespace-pre-wrap break-all">{JSON.stringify(filter, null, 2)}</pre>
                    </div>
                </CardContent>
            </Card>
        </div>
    </div>
</div>
