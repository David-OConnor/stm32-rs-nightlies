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
#[doc = "Field `CGIF1` writer - global interrupt flag clear for channel 1"]
pub type CGIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF1` writer - transfer complete flag clear for channel 1"]
pub type CTCIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF1` writer - half transfer flag clear for channel 1"]
pub type CHTIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF1` writer - transfer error flag clear for channel 1"]
pub type CTEIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CGIF2` writer - global interrupt flag clear for channel 2"]
pub type CGIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF2` writer - transfer complete flag clear for channel 2"]
pub type CTCIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF2` writer - half transfer flag clear for channel 2"]
pub type CHTIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF2` writer - transfer error flag clear for channel 2"]
pub type CTEIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CGIF3` writer - global interrupt flag clear for channel 3"]
pub type CGIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF3` writer - transfer complete flag clear for channel 3"]
pub type CTCIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF3` writer - half transfer flag clear for channel 3"]
pub type CHTIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF3` writer - transfer error flag clear for channel 3"]
pub type CTEIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - global interrupt flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cgif1(&mut self) -> CGIF1_W<0> {
        CGIF1_W::new(self)
    }
    #[doc = "Bit 1 - transfer complete flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> CTCIF1_W<1> {
        CTCIF1_W::new(self)
    }
    #[doc = "Bit 2 - half transfer flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> CHTIF1_W<2> {
        CHTIF1_W::new(self)
    }
    #[doc = "Bit 3 - transfer error flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> CTEIF1_W<3> {
        CTEIF1_W::new(self)
    }
    #[doc = "Bit 4 - global interrupt flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cgif2(&mut self) -> CGIF2_W<4> {
        CGIF2_W::new(self)
    }
    #[doc = "Bit 5 - transfer complete flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> CTCIF2_W<5> {
        CTCIF2_W::new(self)
    }
    #[doc = "Bit 6 - half transfer flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> CHTIF2_W<6> {
        CHTIF2_W::new(self)
    }
    #[doc = "Bit 7 - transfer error flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> CTEIF2_W<7> {
        CTEIF2_W::new(self)
    }
    #[doc = "Bit 8 - global interrupt flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn cgif3(&mut self) -> CGIF3_W<8> {
        CGIF3_W::new(self)
    }
    #[doc = "Bit 9 - transfer complete flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> CTCIF3_W<9> {
        CTCIF3_W::new(self)
    }
    #[doc = "Bit 10 - half transfer flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> CHTIF3_W<10> {
        CHTIF3_W::new(self)
    }
    #[doc = "Bit 11 - transfer error flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> CTEIF3_W<11> {
        CTEIF3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
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
