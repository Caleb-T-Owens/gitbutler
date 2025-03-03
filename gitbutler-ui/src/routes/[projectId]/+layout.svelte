<script lang="ts">
	import { syncToCloud } from '$lib/backend/cloud';
	import { handleMenuActions } from '$lib/backend/menuActions';
	import { BranchService } from '$lib/branches/service';
	import Navigation from '$lib/components/Navigation.svelte';
	import NotOnGitButlerBranch from '$lib/components/NotOnGitButlerBranch.svelte';
	import ProblemLoadingRepo from '$lib/components/ProblemLoadingRepo.svelte';
	import { subscribe as menuSubscribe } from '$lib/menu';
	import * as hotkeys from '$lib/utils/hotkeys';
	import { unsubscribe } from '$lib/utils/random';
	import { BranchController } from '$lib/vbranches/branchController';
	import { BaseBranchService } from '$lib/vbranches/branchStoresCache';
	import { BaseBranch } from '$lib/vbranches/types';
	import { onDestroy, onMount, setContext } from 'svelte';
	import type { LayoutData } from './$types';
	import { goto } from '$app/navigation';

	export let data: LayoutData;

	$: ({
		vbranchService,
		project$,
		projectId,
		baseBranchService,
		gbBranchActive$,
		user$,
		branchService,
		branchController
	} = data);

	$: branchesError = vbranchService.branchesError;
	$: baseBranch = baseBranchService.base;
	$: baseError = baseBranchService.error;

	$: setContext(BranchController, branchController);
	$: setContext(BranchService, branchService);
	$: setContext(BaseBranchService, baseBranchService);
	$: setContext(BaseBranch, baseBranch);

	let intervalId: any;
	handleMenuActions(data.projectId);

	// Once on load and every time the project id changes
	$: if (projectId) setupFetchInterval();

	function setupFetchInterval() {
		baseBranchService.fetchFromTarget();
		clearFetchInterval();
		const intervalMs = 15 * 60 * 1000; // 15 minutes
		intervalId = setInterval(() => baseBranchService.fetchFromTarget(), intervalMs);
	}

	function clearFetchInterval() {
		if (intervalId) clearInterval(intervalId);
	}

	// TODO: Is this an ok way to redirect?
	$: if ($baseBranch === null) goto(`/${projectId}/setup`, { replaceState: true });

	onMount(() => {
		// Once on load and every time the project id changes
		handleMenuActions(data.projectId);
		return unsubscribe(
			menuSubscribe(data.projectId),
			hotkeys.on('Meta+Shift+S', () => syncToCloud(projectId))
		);
	});

	onDestroy(() => clearFetchInterval());
</script>

<!-- forces components to be recreated when projectId changes -->
{#key projectId}
	{#if !$project$}
		<p>Project not found!</p>
	{:else if $baseBranch === null}
		<!-- Be careful, this works because of the redirect above -->
		<slot />
	{:else if $baseError}
		<ProblemLoadingRepo project={$project$} error={$baseError} />
	{:else if $branchesError}
		<ProblemLoadingRepo project={$project$} error={$branchesError} />
	{:else if !$gbBranchActive$ && $baseBranch}
		<NotOnGitButlerBranch project={$project$} baseBranch={$baseBranch} />
	{:else}
		<div class="view-wrap" role="group" on:dragover|preventDefault>
			<Navigation project={$project$} user={$user$} />
			<slot />
		</div>
	{/if}
{/key}

<style>
	.view-wrap {
		position: relative;
		display: flex;
		width: 100%;
	}
</style>
