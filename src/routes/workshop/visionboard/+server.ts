export function GET(): Response {
	return new Response(null, {
		status: 301,
		headers: {
			location: "https://r2.valentinrogg.de/files/PersonalVisionBoard.excalidraw"
		}
	});
}
