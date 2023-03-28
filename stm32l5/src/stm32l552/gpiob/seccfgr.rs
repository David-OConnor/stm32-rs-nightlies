#[doc = "Register `SECCFGR` writer"]
pub struct W(crate::W<SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR_SPEC>;
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
impl From<crate::W<SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC0` writer - I/O pin of Port x secure bit enable"]
pub type SEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC1` writer - I/O pin of Port x secure bit enable"]
pub type SEC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC2` writer - I/O pin of Port x secure bit enable"]
pub type SEC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC3` writer - I/O pin of Port x secure bit enable"]
pub type SEC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC4` writer - I/O pin of Port x secure bit enable"]
pub type SEC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC5` writer - I/O pin of Port x secure bit enable"]
pub type SEC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC6` writer - I/O pin of Port x secure bit enable"]
pub type SEC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC7` writer - I/O pin of Port x secure bit enable"]
pub type SEC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC8` writer - I/O pin of Port x secure bit enable"]
pub type SEC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC9` writer - I/O pin of Port x secure bit enable"]
pub type SEC9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC10` writer - I/O pin of Port x secure bit enable"]
pub type SEC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC11` writer - I/O pin of Port x secure bit enable"]
pub type SEC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC12` writer - I/O pin of Port x secure bit enable"]
pub type SEC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC13` writer - I/O pin of Port x secure bit enable"]
pub type SEC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC14` writer - I/O pin of Port x secure bit enable"]
pub type SEC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SEC15` writer - I/O pin of Port x secure bit enable"]
pub type SEC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec0(&mut self) -> SEC0_W<0> {
        SEC0_W::new(self)
    }
    #[doc = "Bit 1 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<1> {
        SEC1_W::new(self)
    }
    #[doc = "Bit 2 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec2(&mut self) -> SEC2_W<2> {
        SEC2_W::new(self)
    }
    #[doc = "Bit 3 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec3(&mut self) -> SEC3_W<3> {
        SEC3_W::new(self)
    }
    #[doc = "Bit 4 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec4(&mut self) -> SEC4_W<4> {
        SEC4_W::new(self)
    }
    #[doc = "Bit 5 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec5(&mut self) -> SEC5_W<5> {
        SEC5_W::new(self)
    }
    #[doc = "Bit 6 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec6(&mut self) -> SEC6_W<6> {
        SEC6_W::new(self)
    }
    #[doc = "Bit 7 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec7(&mut self) -> SEC7_W<7> {
        SEC7_W::new(self)
    }
    #[doc = "Bit 8 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec8(&mut self) -> SEC8_W<8> {
        SEC8_W::new(self)
    }
    #[doc = "Bit 9 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec9(&mut self) -> SEC9_W<9> {
        SEC9_W::new(self)
    }
    #[doc = "Bit 10 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec10(&mut self) -> SEC10_W<10> {
        SEC10_W::new(self)
    }
    #[doc = "Bit 11 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec11(&mut self) -> SEC11_W<11> {
        SEC11_W::new(self)
    }
    #[doc = "Bit 12 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec12(&mut self) -> SEC12_W<12> {
        SEC12_W::new(self)
    }
    #[doc = "Bit 13 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec13(&mut self) -> SEC13_W<13> {
        SEC13_W::new(self)
    }
    #[doc = "Bit 14 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec14(&mut self) -> SEC14_W<14> {
        SEC14_W::new(self)
    }
    #[doc = "Bit 15 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec15(&mut self) -> SEC15_W<15> {
        SEC15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO secure configuration register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](index.html) module"]
pub struct SECCFGR_SPEC;
impl crate::RegisterSpec for SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [seccfgr::W](W) writer structure"]
impl crate::Writable for SECCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
