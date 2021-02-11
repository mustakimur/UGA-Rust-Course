
optout:     file format elf64-x86-64


Disassembly of section .init:

00000000004004a8 <_init>:
  4004a8:	f3 0f 1e fa          	endbr64 
  4004ac:	48 83 ec 08          	sub    rsp,0x8
  4004b0:	48 8b 05 29 1b 00 00 	mov    rax,QWORD PTR [rip+0x1b29]        # 401fe0 <__gmon_start__>
  4004b7:	48 85 c0             	test   rax,rax
  4004ba:	74 02                	je     4004be <_init+0x16>
  4004bc:	ff d0                	call   rax
  4004be:	48 83 c4 08          	add    rsp,0x8
  4004c2:	c3                   	ret    

Disassembly of section .plt:

00000000004004d0 <printf@plt-0x10>:
  4004d0:	ff 35 1a 1b 00 00    	push   QWORD PTR [rip+0x1b1a]        # 401ff0 <_GLOBAL_OFFSET_TABLE_+0x8>
  4004d6:	ff 25 1c 1b 00 00    	jmp    QWORD PTR [rip+0x1b1c]        # 401ff8 <_GLOBAL_OFFSET_TABLE_+0x10>
  4004dc:	90                   	nop
  4004dd:	90                   	nop
  4004de:	90                   	nop
  4004df:	90                   	nop

00000000004004e0 <printf@plt>:
  4004e0:	ff 25 1a 1b 00 00    	jmp    QWORD PTR [rip+0x1b1a]        # 402000 <printf@GLIBC_2.2.5>
  4004e6:	68 00 00 00 00       	push   0x0
  4004eb:	e9 e0 ff ff ff       	jmp    4004d0 <_init+0x28>

00000000004004f0 <strcpy@plt>:
  4004f0:	ff 25 12 1b 00 00    	jmp    QWORD PTR [rip+0x1b12]        # 402008 <strcpy@GLIBC_2.2.5>
  4004f6:	68 01 00 00 00       	push   0x1
  4004fb:	e9 d0 ff ff ff       	jmp    4004d0 <_init+0x28>

0000000000400500 <memset@plt>:
  400500:	ff 25 0a 1b 00 00    	jmp    QWORD PTR [rip+0x1b0a]        # 402010 <memset@GLIBC_2.2.5>
  400506:	68 02 00 00 00       	push   0x2
  40050b:	e9 c0 ff ff ff       	jmp    4004d0 <_init+0x28>

Disassembly of section .text:

0000000000400510 <_start>:
  400510:	f3 0f 1e fa          	endbr64 
  400514:	31 ed                	xor    ebp,ebp
  400516:	49 89 d1             	mov    r9,rdx
  400519:	5e                   	pop    rsi
  40051a:	48 89 e2             	mov    rdx,rsp
  40051d:	48 83 e4 f0          	and    rsp,0xfffffffffffffff0
  400521:	50                   	push   rax
  400522:	54                   	push   rsp
  400523:	4c 8d 05 f6 01 00 00 	lea    r8,[rip+0x1f6]        # 400720 <__libc_csu_fini>
  40052a:	48 8d 0d 7f 01 00 00 	lea    rcx,[rip+0x17f]        # 4006b0 <__libc_csu_init>
  400531:	48 8d 3d 28 01 00 00 	lea    rdi,[rip+0x128]        # 400660 <main>
  400538:	ff 15 9a 1a 00 00    	call   QWORD PTR [rip+0x1a9a]        # 401fd8 <__libc_start_main@GLIBC_2.2.5>
  40053e:	f4                   	hlt    
  40053f:	90                   	nop

0000000000400540 <_dl_relocate_static_pie>:
  400540:	f3 0f 1e fa          	endbr64 
  400544:	c3                   	ret    
  400545:	66 66 2e 0f 1f 84 00 	data16 nop WORD PTR cs:[rax+rax*1+0x0]
  40054c:	00 00 00 00 

0000000000400550 <deregister_tm_clones>:
  400550:	b8 28 20 40 00       	mov    eax,0x402028
  400555:	48 3d 28 20 40 00    	cmp    rax,0x402028
  40055b:	74 13                	je     400570 <deregister_tm_clones+0x20>
  40055d:	b8 00 00 00 00       	mov    eax,0x0
  400562:	48 85 c0             	test   rax,rax
  400565:	74 09                	je     400570 <deregister_tm_clones+0x20>
  400567:	bf 28 20 40 00       	mov    edi,0x402028
  40056c:	ff e0                	jmp    rax
  40056e:	66 90                	xchg   ax,ax
  400570:	c3                   	ret    
  400571:	66 66 2e 0f 1f 84 00 	data16 nop WORD PTR cs:[rax+rax*1+0x0]
  400578:	00 00 00 00 
  40057c:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]

0000000000400580 <register_tm_clones>:
  400580:	be 28 20 40 00       	mov    esi,0x402028
  400585:	48 81 ee 28 20 40 00 	sub    rsi,0x402028
  40058c:	48 89 f0             	mov    rax,rsi
  40058f:	48 c1 ee 3f          	shr    rsi,0x3f
  400593:	48 c1 f8 03          	sar    rax,0x3
  400597:	48 01 c6             	add    rsi,rax
  40059a:	48 d1 fe             	sar    rsi,1
  40059d:	74 11                	je     4005b0 <register_tm_clones+0x30>
  40059f:	b8 00 00 00 00       	mov    eax,0x0
  4005a4:	48 85 c0             	test   rax,rax
  4005a7:	74 07                	je     4005b0 <register_tm_clones+0x30>
  4005a9:	bf 28 20 40 00       	mov    edi,0x402028
  4005ae:	ff e0                	jmp    rax
  4005b0:	c3                   	ret    
  4005b1:	66 66 2e 0f 1f 84 00 	data16 nop WORD PTR cs:[rax+rax*1+0x0]
  4005b8:	00 00 00 00 
  4005bc:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]

00000000004005c0 <__do_global_dtors_aux>:
  4005c0:	f3 0f 1e fa          	endbr64 
  4005c4:	80 3d 5d 1a 00 00 00 	cmp    BYTE PTR [rip+0x1a5d],0x0        # 402028 <completed.8060>
  4005cb:	75 13                	jne    4005e0 <__do_global_dtors_aux+0x20>
  4005cd:	55                   	push   rbp
  4005ce:	48 89 e5             	mov    rbp,rsp
  4005d1:	e8 7a ff ff ff       	call   400550 <deregister_tm_clones>
  4005d6:	c6 05 4b 1a 00 00 01 	mov    BYTE PTR [rip+0x1a4b],0x1        # 402028 <completed.8060>
  4005dd:	5d                   	pop    rbp
  4005de:	c3                   	ret    
  4005df:	90                   	nop
  4005e0:	c3                   	ret    
  4005e1:	66 66 2e 0f 1f 84 00 	data16 nop WORD PTR cs:[rax+rax*1+0x0]
  4005e8:	00 00 00 00 
  4005ec:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]

00000000004005f0 <frame_dummy>:
  4005f0:	f3 0f 1e fa          	endbr64 
  4005f4:	eb 8a                	jmp    400580 <register_tm_clones>
  4005f6:	66 2e 0f 1f 84 00 00 	nop    WORD PTR cs:[rax+rax*1+0x0]
  4005fd:	00 00 00 

0000000000400600 <encrypts>:
  400600:	55                   	push   rbp
  400601:	48 89 e5             	mov    rbp,rsp
  400604:	48 83 ec 10          	sub    rsp,0x10
  400608:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
  40060c:	48 83 7d f8 00       	cmp    QWORD PTR [rbp-0x8],0x0
  400611:	0f 84 21 00 00 00    	je     400638 <encrypts+0x38>
  400617:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
  40061b:	48 05 0a 00 00 00    	add    rax,0xa
  400621:	0f be c0             	movsx  eax,al
  400624:	99                   	cdq    
  400625:	b9 80 00 00 00       	mov    ecx,0x80
  40062a:	f7 f9                	idiv   ecx
  40062c:	48 63 f2             	movsxd rsi,edx
  40062f:	48 89 75 f8          	mov    QWORD PTR [rbp-0x8],rsi
  400633:	e9 d4 ff ff ff       	jmp    40060c <encrypts+0xc>
  400638:	48 8b 75 f8          	mov    rsi,QWORD PTR [rbp-0x8]
  40063c:	48 bf 3c 07 40 00 00 	movabs rdi,0x40073c
  400643:	00 00 00 
  400646:	b0 00                	mov    al,0x0
  400648:	e8 93 fe ff ff       	call   4004e0 <printf@plt>
  40064d:	48 83 c4 10          	add    rsp,0x10
  400651:	5d                   	pop    rbp
  400652:	c3                   	ret    
  400653:	66 2e 0f 1f 84 00 00 	nop    WORD PTR cs:[rax+rax*1+0x0]
  40065a:	00 00 00 
  40065d:	0f 1f 00             	nop    DWORD PTR [rax]

0000000000400660 <main>:
  400660:	55                   	push   rbp
  400661:	48 89 e5             	mov    rbp,rsp
  400664:	48 83 ec 30          	sub    rsp,0x30
  400668:	48 8d 45 f0          	lea    rax,[rbp-0x10]
  40066c:	be 40 07 40 00       	mov    esi,0x400740
  400671:	48 89 c7             	mov    rdi,rax
  400674:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
  400678:	e8 73 fe ff ff       	call   4004f0 <strcpy@plt>
  40067d:	48 8b 7d e8          	mov    rdi,QWORD PTR [rbp-0x18]
  400681:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
  400685:	e8 76 ff ff ff       	call   400600 <encrypts>
  40068a:	31 c9                	xor    ecx,ecx
  40068c:	48 8d 7d f0          	lea    rdi,[rbp-0x10]
  400690:	89 ce                	mov    esi,ecx
  400692:	ba 10 00 00 00       	mov    edx,0x10
  400697:	89 4d dc             	mov    DWORD PTR [rbp-0x24],ecx
  40069a:	e8 61 fe ff ff       	call   400500 <memset@plt>
  40069f:	8b 45 dc             	mov    eax,DWORD PTR [rbp-0x24]
  4006a2:	48 83 c4 30          	add    rsp,0x30
  4006a6:	5d                   	pop    rbp
  4006a7:	c3                   	ret    
  4006a8:	0f 1f 84 00 00 00 00 	nop    DWORD PTR [rax+rax*1+0x0]
  4006af:	00 

00000000004006b0 <__libc_csu_init>:
  4006b0:	f3 0f 1e fa          	endbr64 
  4006b4:	41 57                	push   r15
  4006b6:	4c 8d 3d 43 17 00 00 	lea    r15,[rip+0x1743]        # 401e00 <__frame_dummy_init_array_entry>
  4006bd:	41 56                	push   r14
  4006bf:	49 89 d6             	mov    r14,rdx
  4006c2:	41 55                	push   r13
  4006c4:	49 89 f5             	mov    r13,rsi
  4006c7:	41 54                	push   r12
  4006c9:	41 89 fc             	mov    r12d,edi
  4006cc:	55                   	push   rbp
  4006cd:	48 8d 2d 34 17 00 00 	lea    rbp,[rip+0x1734]        # 401e08 <_DYNAMIC>
  4006d4:	53                   	push   rbx
  4006d5:	4c 29 fd             	sub    rbp,r15
  4006d8:	48 83 ec 08          	sub    rsp,0x8
  4006dc:	e8 c7 fd ff ff       	call   4004a8 <_init>
  4006e1:	48 c1 fd 03          	sar    rbp,0x3
  4006e5:	74 1f                	je     400706 <__libc_csu_init+0x56>
  4006e7:	31 db                	xor    ebx,ebx
  4006e9:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
  4006f0:	4c 89 f2             	mov    rdx,r14
  4006f3:	4c 89 ee             	mov    rsi,r13
  4006f6:	44 89 e7             	mov    edi,r12d
  4006f9:	41 ff 14 df          	call   QWORD PTR [r15+rbx*8]
  4006fd:	48 83 c3 01          	add    rbx,0x1
  400701:	48 39 dd             	cmp    rbp,rbx
  400704:	75 ea                	jne    4006f0 <__libc_csu_init+0x40>
  400706:	48 83 c4 08          	add    rsp,0x8
  40070a:	5b                   	pop    rbx
  40070b:	5d                   	pop    rbp
  40070c:	41 5c                	pop    r12
  40070e:	41 5d                	pop    r13
  400710:	41 5e                	pop    r14
  400712:	41 5f                	pop    r15
  400714:	c3                   	ret    
  400715:	66 66 2e 0f 1f 84 00 	data16 nop WORD PTR cs:[rax+rax*1+0x0]
  40071c:	00 00 00 00 

0000000000400720 <__libc_csu_fini>:
  400720:	f3 0f 1e fa          	endbr64 
  400724:	c3                   	ret    

Disassembly of section .fini:

0000000000400728 <_fini>:
  400728:	f3 0f 1e fa          	endbr64 
  40072c:	48 83 ec 08          	sub    rsp,0x8
  400730:	48 83 c4 08          	add    rsp,0x8
  400734:	c3                   	ret    
