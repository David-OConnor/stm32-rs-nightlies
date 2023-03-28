#[doc = "Register `IFCR` writer"]
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
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
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CGIF1` writer - Clear channel 1 global interrupt flag"]
pub type CGIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF1` writer - Clear channel 1 transfer complete flag"]
pub type CTCIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF1` writer - Clear channel 1 half transfer flag"]
pub type CHTIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF1` writer - Clear channel 1 transfer error flag"]
pub type CTEIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CGIF2` writer - Clear channel 2 global interrupt flag"]
pub type CGIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF2` writer - Clear channel 2 transfer complete flag"]
pub type CTCIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF2` writer - Clear channel 2 half transfer flag"]
pub type CHTIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF2` writer - Clear channel 2 transfer error flag"]
pub type CTEIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CGIF3` writer - Clear channel 3 global interrupt flag"]
pub type CGIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF3` writer - Clear channel 3 transfer complete flag"]
pub type CTCIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF3` writer - Clear channel 3 half transfer flag"]
pub type CHTIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF3` writer - Clear channel 3 transfer error flag"]
pub type CTEIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CGIF4` writer - Clear channel 4 global interrupt flag"]
pub type CGIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF4` writer - Clear channel 4 transfer complete flag"]
pub type CTCIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF4` writer - Clear channel 4 half transfer flag"]
pub type CHTIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF4` writer - Clear channel 4 transfer error flag"]
pub type CTEIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CGIF5` writer - Clear channel 5 global interrupt flag"]
pub type CGIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF5` writer - Clear channel 5 transfer complete flag"]
pub type CTCIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF5` writer - Clear channel 5 half transfer flag"]
pub type CHTIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF5` writer - Clear channel 5 transfer error flag"]
pub type CTEIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CGIF6` writer - Clear channel 6 global interrupt flag"]
pub type CGIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF6` writer - Clear channel 6 transfer complete flag"]
pub type CTCIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF6` writer - Clear channel 6 half transfer flag"]
pub type CHTIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF6` writer - Clear channel 6 transfer error flag"]
pub type CTEIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CGIF7` writer - Clear channel 7 global interrupt flag"]
pub type CGIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF7` writer - Clear channel 7 transfer complete flag"]
pub type CTCIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF7` writer - Clear channel 7 half transfer flag"]
pub type CHTIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF7` writer - Clear channel 7 transfer error flag"]
pub type CTEIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear channel 1 global interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cgif1(&mut self) -> CGIF1_W<0> {
        CGIF1_W::new(self)
    }
    #[doc = "Bit 1 - Clear channel 1 transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> CTCIF1_W<1> {
        CTCIF1_W::new(self)
    }
    #[doc = "Bit 2 - Clear channel 1 half transfer flag"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> CHTIF1_W<2> {
        CHTIF1_W::new(self)
    }
    #[doc = "Bit 3 - Clear channel 1 transfer error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> CTEIF1_W<3> {
        CTEIF1_W::new(self)
    }
    #[doc = "Bit 4 - Clear channel 2 global interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cgif2(&mut self) -> CGIF2_W<4> {
        CGIF2_W::new(self)
    }
    #[doc = "Bit 5 - Clear channel 2 transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> CTCIF2_W<5> {
        CTCIF2_W::new(self)
    }
    #[doc = "Bit 6 - Clear channel 2 half transfer flag"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> CHTIF2_W<6> {
        CHTIF2_W::new(self)
    }
    #[doc = "Bit 7 - Clear channel 2 transfer error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> CTEIF2_W<7> {
        CTEIF2_W::new(self)
    }
    #[doc = "Bit 8 - Clear channel 3 global interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cgif3(&mut self) -> CGIF3_W<8> {
        CGIF3_W::new(self)
    }
    #[doc = "Bit 9 - Clear channel 3 transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> CTCIF3_W<9> {
        CTCIF3_W::new(self)
    }
    #[doc = "Bit 10 - Clear channel 3 half transfer flag"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> CHTIF3_W<10> {
        CHTIF3_W::new(self)
    }
    #[doc = "Bit 11 - Clear channel 3 transfer error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> CTEIF3_W<11> {
        CTEIF3_W::new(self)
    }
    #[doc = "Bit 12 - Clear channel 4 global interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cgif4(&mut self) -> CGIF4_W<12> {
        CGIF4_W::new(self)
    }
    #[doc = "Bit 13 - Clear channel 4 transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif4(&mut self) -> CTCIF4_W<13> {
        CTCIF4_W::new(self)
    }
    #[doc = "Bit 14 - Clear channel 4 half transfer flag"]
    #[inline(always)]
    #[must_use]
    pub fn chtif4(&mut self) -> CHTIF4_W<14> {
        CHTIF4_W::new(self)
    }
    #[doc = "Bit 15 - Clear channel 4 transfer error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cteif4(&mut self) -> CTEIF4_W<15> {
        CTEIF4_W::new(self)
    }
    #[doc = "Bit 16 - Clear channel 5 global interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cgif5(&mut self) -> CGIF5_W<16> {
        CGIF5_W::new(self)
    }
    #[doc = "Bit 17 - Clear channel 5 transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif5(&mut self) -> CTCIF5_W<17> {
        CTCIF5_W::new(self)
    }
    #[doc = "Bit 18 - Clear channel 5 half transfer flag"]
    #[inline(always)]
    #[must_use]
    pub fn chtif5(&mut self) -> CHTIF5_W<18> {
        CHTIF5_W::new(self)
    }
    #[doc = "Bit 19 - Clear channel 5 transfer error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cteif5(&mut self) -> CTEIF5_W<19> {
        CTEIF5_W::new(self)
    }
    #[doc = "Bit 20 - Clear channel 6 global interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cgif6(&mut self) -> CGIF6_W<20> {
        CGIF6_W::new(self)
    }
    #[doc = "Bit 21 - Clear channel 6 transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif6(&mut self) -> CTCIF6_W<21> {
        CTCIF6_W::new(self)
    }
    #[doc = "Bit 22 - Clear channel 6 half transfer flag"]
    #[inline(always)]
    #[must_use]
    pub fn chtif6(&mut self) -> CHTIF6_W<22> {
        CHTIF6_W::new(self)
    }
    #[doc = "Bit 23 - Clear channel 6 transfer error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cteif6(&mut self) -> CTEIF6_W<23> {
        CTEIF6_W::new(self)
    }
    #[doc = "Bit 24 - Clear channel 7 global interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cgif7(&mut self) -> CGIF7_W<24> {
        CGIF7_W::new(self)
    }
    #[doc = "Bit 25 - Clear channel 7 transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif7(&mut self) -> CTCIF7_W<25> {
        CTCIF7_W::new(self)
    }
    #[doc = "Bit 26 - Clear channel 7 half transfer flag"]
    #[inline(always)]
    #[must_use]
    pub fn chtif7(&mut self) -> CHTIF7_W<26> {
        CHTIF7_W::new(self)
    }
    #[doc = "Bit 27 - Clear channel 7 transfer error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cteif7(&mut self) -> CTEIF7_W<27> {
        CTEIF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "high interrupt status register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifcr::W](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
