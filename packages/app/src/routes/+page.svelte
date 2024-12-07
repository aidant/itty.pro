<script lang="ts">
    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault();

        const data = new FormData(event.target as HTMLFormElement);

        const long = data.get("url")?.toString();
        const short = await fetch("/", { method: "POST", body: long }).then(
            (response) => response.text(),
        );

        await navigator.clipboard.writeText(short);
    };
</script>

<main
    class="h-full w-full flex flex-col justify-center items-center font-sans text-fuchsia-950"
>
    <form class="flex flex-col gap-3 max-w-md w-full" on:submit={handleSubmit}>
        <label>
            Url
            <input
                type="url"
                name="url"
                class="px-3 py-2 rounded-lg block mt-1 w-full border-fuchsia-200 shadow-md shadow-fuchsia-600/10 transition focus:ring-3 focus:ring-fuchsia-200/50 focus:border-fuchsia-400"
            />
        </label>
        <button
            class="px-3 py-2 rounded-lg block bg-fuchsia-700 text-white border-1 border-fuchsia-200 shadow-md shadow-fuchsia-600/10 transition focus:ring-3 fosuc:ring-fuchsia-200/50 focus-visible:outline-none focus:border-fuchsia-400 hover:bg-fuchsia-800"
            >Shorten</button
        >
    </form>
</main>
