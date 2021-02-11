
optout:     file format elf64-x86-64


Disassembly of section .init:

0000000000400430 <_init>:
  400430:	f3 0f 1e fa          	endbr64 
  400434:	48 83 ec 08          	sub    rsp,0x8
  400438:	48 8b 05 a1 1b 00 00 	mov    rax,QWORD PTR [rip+0x1ba1]        # 401fe0 <__gmon_start__>
  40043f:	48 85 c0             	test   rax,rax
  400442:	74 02                	je     400446 <_init+0x16>
  400444:	ff d0                	call   rax
  400446:	48 83 c4 08          	add    rsp,0x8
  40044a:	c3                   	ret    

Disassembly of section .plt:

0000000000400450 <puts@plt-0x10>:
  400450:	ff 35 9a 1b 00 00    	push   QWORD PTR [rip+0x1b9a]        # 401ff0 <_GLOBAL_OFFSET_TABLE_+0x8>
  400456:	ff 25 9c 1b 00 00    	jmp    QWORD PTR [rip+0x1b9c]        # 401ff8 <_GLOBAL_OFFSET_TABLE_+0x10>
  40045c:	90                   	nop
  40045d:	90                   	nop
  40045e:	90                   	nop
  40045f:	90                   	nop

0000000000400460 <puts@plt>:
  400460:	ff 25 9a 1b 00 00    	jmp    QWORD PTR [rip+0x1b9a]        # 402000 <puts@GLIBC_2.2.5>
  400466:	68 00 00 00 00       	push   0x0
  40046b:	e9 e0 ff ff ff       	jmp    400450 <_init+0x20>

Disassembly of section .text:

0000000000400470 <_start>:
  400470:	f3 0f 1e fa          	endbr64 
  400474:	31 ed                	xor    ebp,ebp
  400476:	49 89 d1             	mov    r9,rdx
  400479:	5e                   	pop    rsi
  40047a:	48 89 e2             	mov    rdx,rsp
  40047d:	48 83 e4 f0          	and    rsp,0xfffffffffffffff0
  400481:	50                   	push   rax
  400482:	54                   	push   rsp
  400483:	4c 8d 05 b6 01 00 00 	lea    r8,[rip+0x1b6]        # 400640 <__libc_csu_fini>
  40048a:	48 8d 0d 3f 01 00 00 	lea    rcx,[rip+0x13f]        # 4005d0 <__libc_csu_init>
  400491:	48 8d 3d 08 01 00 00 	lea    rdi,[rip+0x108]        # 4005a0 <main>
  400498:	ff 15 3a 1b 00 00    	call   QWORD PTR [rip+0x1b3a]        # 401fd8 <__libc_start_main@GLIBC_2.2.5>
  40049e:	f4                   	hlt    
  40049f:	90                   	nop

00000000004004a0 <_dl_relocate_static_pie>:
  4004a0:	f3 0f 1e fa          	endbr64 
  4004a4:	c3                   	ret    
  4004a5:	66 66 2e 0f 1f 84 00 	data16 nop WORD PTR cs:[rax+rax*1+0x0]
  4004ac:	00 00 00 00 

00000000004004b0 <deregister_tm_clones>:
  4004b0:	b8 18 20 40 00       	mov    eax,0x402018
  4004b5:	48 3d 18 20 40 00    	cmp    rax,0x402018
  4004bb:	74 13                	je     4004d0 <deregister_tm_clones+0x20>
  4004bd:	b8 00 00 00 00       	mov    eax,0x0
  4004c2:	48 85 c0             	test   rax,rax
  4004c5:	74 09                	je     4004d0 <deregister_tm_clones+0x20>
  4004c7:	bf 18 20 40 00       	mov    edi,0x402018
  4004cc:	ff e0                	jmp    rax
  4004ce:	66 90                	xchg   ax,ax
  4004d0:	c3                   	ret    
  4004d1:	66 66 2e 0f 1f 84 00 	data16 nop WORD PTR cs:[rax+rax*1+0x0]
  4004d8:	00 00 00 00 
  4004dc:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]

00000000004004e0 <register_tm_clones>:
  4004e0:	be 18 20 40 00       	mov    esi,0x402018
  4004e5:	48 81 ee 18 20 40 00 	sub    rsi,0x402018
  4004ec:	48 89 f0             	mov    rax,rsi
  4004ef:	48 c1 ee 3f          	shr    rsi,0x3f
  4004f3:	48 c1 f8 03          	sar    rax,0x3
  4004f7:	48 01 c6             	add    rsi,rax
  4004fa:	48 d1 fe             	sar    rsi,1
  4004fd:	74 11                	je     400510 <register_tm_clones+0x30>
  4004ff:	b8 00 00 00 00       	mov    eax,0x0
  400504:	48 85 c0             	test   rax,rax
  400507:	74 07                	je     400510 <register_tm_clones+0x30>
  400509:	bf 18 20 40 00       	mov    edi,0x402018
  40050e:	ff e0                	jmp    rax
  400510:	c3                   	ret    
  400511:	66 66 2e 0f 1f 84 00 	data16 nop WORD PTR cs:[rax+rax*1+0x0]
  400518:	00 00 00 00 
  40051c:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]

0000000000400520 <__do_global_dtors_aux>:
  400520:	f3 0f 1e fa          	endbr64 
  400524:	80 3d ed 1a 00 00 00 	cmp    BYTE PTR [rip+0x1aed],0x0        # 402018 <completed.8060>
  40052b:	75 13                	jne    400540 <__do_global_dtors_aux+0x20>
  40052d:	55                   	push   rbp
  40052e:	48 89 e5             	mov    rbp,rsp
  400531:	e8 7a ff ff ff       	call   4004b0 <deregister_tm_clones>
  400536:	c6 05 db 1a 00 00 01 	mov    BYTE PTR [rip+0x1adb],0x1        # 402018 <completed.8060>
  40053d:	5d                   	pop    rbp
  40053e:	c3                   	ret    
  40053f:	90                   	nop
  400540:	c3                   	ret    
  400541:	66 66 2e 0f 1f 84 00 	data16 nop WORD PTR cs:[rax+rax*1+0x0]
  400548:	00 00 00 00 
  40054c:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]

0000000000400550 <frame_dummy>:
  400550:	f3 0f 1e fa          	endbr64 
  400554:	eb 8a                	jmp    4004e0 <register_tm_clones>
  400556:	66 2e 0f 1f 84 00 00 	nop    WORD PTR cs:[rax+rax*1+0x0]
  40055d:	00 00 00 

0000000000400560 <encrypts>:
  400560:	48 85 ff             	test   rdi,rdi
  400563:	74 24                	je     400589 <encrypts+0x29>
  400565:	66 2e 0f 1f 84 00 00 	nop    WORD PTR cs:[rax+rax*1+0x0]
  40056c:	00 00 00 
  40056f:	90                   	nop
  400570:	83 c7 0a             	add    edi,0xa
  400573:	40 0f be c7          	movsx  eax,dil
  400577:	8d 48 7f             	lea    ecx,[rax+0x7f]
  40057a:	85 c0                	test   eax,eax
  40057c:	0f 49 c8             	cmovns ecx,eax
  40057f:	83 e1 80             	and    ecx,0xffffff80
  400582:	29 c8                	sub    eax,ecx
  400584:	48 63 f8             	movsxd rdi,eax
  400587:	75 e7                	jne    400570 <encrypts+0x10>
  400589:	50                   	push   rax
  40058a:	31 ff                	xor    edi,edi
  40058c:	e8 cf fe ff ff       	call   400460 <puts@plt>
  400591:	58                   	pop    rax
  400592:	c3                   	ret    
  400593:	66 2e 0f 1f 84 00 00 	nop    WORD PTR cs:[rax+rax*1+0x0]
  40059a:	00 00 00 
  40059d:	0f 1f 00             	nop    DWORD PTR [rax]

00000000004005a0 <main>:
  4005a0:	48 83 ec 18          	sub    rsp,0x18
  4005a4:	48 b8 77 68 61 74 65 	movabs rax,0x7265766574616877
  4005ab:	76 65 72 
  4005ae:	48 89 04 24          	mov    QWORD PTR [rsp],rax
  4005b2:	c6 44 24 08 00       	mov    BYTE PTR [rsp+0x8],0x0
  4005b7:	48 89 e7             	mov    rdi,rsp
  4005ba:	e8 a1 ff ff ff       	call   400560 <encrypts>
  4005bf:	31 c0                	xor    eax,eax
  4005c1:	48 83 c4 18          	add    rsp,0x18
  4005c5:	c3                   	ret    
  4005c6:	66 2e 0f 1f 84 00 00 	nop    WORD PTR cs:[rax+rax*1+0x0]
  4005cd:	00 00 00 

00000000004005d0 <__libc_csu_init>:
  4005d0:	f3 0f 1e fa          	endbr64 
  4005d4:	41 57                	push   r15
  4005d6:	4c 8d 3d 23 18 00 00 	lea    r15,[rip+0x1823]        # 401e00 <__frame_dummy_init_array_entry>
  4005dd:	41 56                	push   r14
  4005df:	49 89 d6             	mov    r14,rdx
  4005e2:	41 55                	push   r13
  4005e4:	49 89 f5             	mov    r13,rsi
  4005e7:	41 54                	push   r12
  4005e9:	41 89 fc             	mov    r12d,edi
  4005ec:	55                   	push   rbp
  4005ed:	48 8d 2d 14 18 00 00 	lea    rbp,[rip+0x1814]        # 401e08 <_DYNAMIC>
  4005f4:	53                   	push   rbx
  4005f5:	4c 29 fd             	sub    rbp,r15
  4005f8:	48 83 ec 08          	sub    rsp,0x8
  4005fc:	e8 2f fe ff ff       	call   400430 <_init>
  400601:	48 c1 fd 03          	sar    rbp,0x3
  400605:	74 1f                	je     400626 <__libc_csu_init+0x56>
  400607:	31 db                	xor    ebx,ebx
  400609:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
  400610:	4c 89 f2             	mov    rdx,r14
  400613:	4c 89 ee             	mov    rsi,r13
  400616:	44 89 e7             	mov    edi,r12d
  400619:	41 ff 14 df          	call   QWORD PTR [r15+rbx*8]
  40061d:	48 83 c3 01          	add    rbx,0x1
  400621:	48 39 dd             	cmp    rbp,rbx
  400624:	75 ea                	jne    400610 <__libc_csu_init+0x40>
  400626:	48 83 c4 08          	add    rsp,0x8
  40062a:	5b                   	pop    rbx
  40062b:	5d                   	pop    rbp
  40062c:	41 5c                	pop    r12
  40062e:	41 5d                	pop    r13
  400630:	41 5e                	pop    r14
  400632:	41 5f                	pop    r15
  400634:	c3                   	ret    
  400635:	66 66 2e 0f 1f 84 00 	data16 nop WORD PTR cs:[rax+rax*1+0x0]
  40063c:	00 00 00 00 

0000000000400640 <__libc_csu_fini>:
  400640:	f3 0f 1e fa          	endbr64 
  400644:	c3                   	ret    

Disassembly of section .fini:

0000000000400648 <_fini>:
  400648:	f3 0f 1e fa          	endbr64 
  40064c:	48 83 ec 08          	sub    rsp,0x8
  400650:	48 83 c4 08          	add    rsp,0x8
  400654:	c3                   	ret    
