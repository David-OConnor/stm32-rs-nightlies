#[doc = "Register `ICACHE_CRR0` reader"]
pub struct R(crate::R<ICACHE_CRR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_CRR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_CRR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_CRR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_CRR0` writer"]
pub struct W(crate::W<ICACHE_CRR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_CRR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ICACHE_CRR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_CRR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASEADDR` reader - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \\[28:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
pub type BASEADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BASEADDR` writer - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \\[28:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
pub type BASEADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICACHE_CRR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSIZE` reader - size for region x"]
pub type RSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSIZE` writer - size for region x"]
pub type RSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICACHE_CRR0_SPEC, u8, u8, 3, O>;
#[doc = "Field `REN` reader - enable for region x"]
pub type REN_R = crate::BitReader<bool>;
#[doc = "Field `REN` writer - enable for region x"]
pub type REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_CRR0_SPEC, bool, O>;
#[doc = "Field `REMAPADDR` reader - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \\[31:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
pub type REMAPADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REMAPADDR` writer - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \\[31:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
pub type REMAPADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ICACHE_CRR0_SPEC, u16, u16, 11, O>;
#[doc = "Field `MSTSEL` reader - AHB cache master selection for region x"]
pub type MSTSEL_R = crate::BitReader<bool>;
#[doc = "Field `MSTSEL` writer - AHB cache master selection for region x"]
pub type MSTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_CRR0_SPEC, bool, O>;
#[doc = "Field `HBURST` reader - output burst type for region x"]
pub type HBURST_R = crate::BitReader<bool>;
#[doc = "Field `HBURST` writer - output burst type for region x"]
pub type HBURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_CRR0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \\[28:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:11 - size for region x"]
    #[inline(always)]
    pub fn rsize(&self) -> RSIZE_R {
        RSIZE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 15 - enable for region x"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \\[31:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
    #[inline(always)]
    pub fn remapaddr(&self) -> REMAPADDR_R {
        REMAPADDR_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - AHB cache master selection for region x"]
    #[inline(always)]
    pub fn mstsel(&self) -> MSTSEL_R {
        MSTSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - output burst type for region x"]
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \\[28:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn baseaddr(&mut self) -> BASEADDR_W<0> {
        BASEADDR_W::new(self)
    }
    #[doc = "Bits 9:11 - size for region x"]
    #[inline(always)]
    #[must_use]
    pub fn rsize(&mut self) -> RSIZE_W<9> {
        RSIZE_W::new(self)
    }
    #[doc = "Bit 15 - enable for region x"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<15> {
        REN_W::new(self)
    }
    #[doc = "Bits 16:26 - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \\[31:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn remapaddr(&mut self) -> REMAPADDR_W<16> {
        REMAPADDR_W::new(self)
    }
    #[doc = "Bit 28 - AHB cache master selection for region x"]
    #[inline(always)]
    #[must_use]
    pub fn mstsel(&mut self) -> MSTSEL_W<28> {
        MSTSEL_W::new(self)
    }
    #[doc = "Bit 31 - output burst type for region x"]
    #[inline(always)]
    #[must_use]
    pub fn hburst(&mut self) -> HBURST_W<31> {
        HBURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICACHE region 0 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_crr0](index.html) module"]
pub struct ICACHE_CRR0_SPEC;
impl crate::RegisterSpec for ICACHE_CRR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_crr0::R](R) reader structure"]
impl crate::Readable for ICACHE_CRR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_crr0::W](W) writer structure"]
impl crate::Writable for ICACHE_CRR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICACHE_CRR0 to value 0x0200"]
impl crate::Resettable for ICACHE_CRR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
