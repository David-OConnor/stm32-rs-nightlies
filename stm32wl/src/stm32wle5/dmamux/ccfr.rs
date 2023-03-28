#[doc = "Register `CCFR` writer"]
pub struct W(crate::W<CCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFR_SPEC>;
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
impl From<crate::W<CCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CSOF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSOF0_AW {
    #[doc = "1: Clear synchronization flag"]
    Clear = 1,
}
impl From<CSOF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CSOF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSOF0` writer - CSOF0"]
pub type CSOF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, CSOF0_AW, O>;
impl<'a, const O: u8> CSOF0_W<'a, O> {
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSOF0_AW::Clear)
    }
}
#[doc = "Field `CSOF1` writer - CSOF1"]
pub use CSOF0_W as CSOF1_W;
#[doc = "Field `CSOF2` writer - CSOF2"]
pub use CSOF0_W as CSOF2_W;
#[doc = "Field `CSOF3` writer - CSOF3"]
pub use CSOF0_W as CSOF3_W;
#[doc = "Field `CSOF4` writer - CSOF4"]
pub use CSOF0_W as CSOF4_W;
#[doc = "Field `CSOF5` writer - CSOF5"]
pub use CSOF0_W as CSOF5_W;
#[doc = "Field `CSOF6` writer - CSOF6"]
pub use CSOF0_W as CSOF6_W;
#[doc = "Field `CSOF7` writer - CSOF7"]
pub use CSOF0_W as CSOF7_W;
#[doc = "Field `CSOF8` writer - CSOF8"]
pub use CSOF0_W as CSOF8_W;
#[doc = "Field `CSOF9` writer - CSOF9"]
pub use CSOF0_W as CSOF9_W;
#[doc = "Field `CSOF10` writer - CSOF10"]
pub use CSOF0_W as CSOF10_W;
#[doc = "Field `CSOF11` writer - CSOF11"]
pub use CSOF0_W as CSOF11_W;
#[doc = "Field `CSOF12` writer - CSOF12"]
pub use CSOF0_W as CSOF12_W;
#[doc = "Field `CSOF13` writer - CSOF13"]
pub use CSOF0_W as CSOF13_W;
impl W {
    #[doc = "Bit 0 - CSOF0"]
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> CSOF0_W<0> {
        CSOF0_W::new(self)
    }
    #[doc = "Bit 1 - CSOF1"]
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> CSOF1_W<1> {
        CSOF1_W::new(self)
    }
    #[doc = "Bit 2 - CSOF2"]
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> CSOF2_W<2> {
        CSOF2_W::new(self)
    }
    #[doc = "Bit 3 - CSOF3"]
    #[inline(always)]
    #[must_use]
    pub fn csof3(&mut self) -> CSOF3_W<3> {
        CSOF3_W::new(self)
    }
    #[doc = "Bit 4 - CSOF4"]
    #[inline(always)]
    #[must_use]
    pub fn csof4(&mut self) -> CSOF4_W<4> {
        CSOF4_W::new(self)
    }
    #[doc = "Bit 5 - CSOF5"]
    #[inline(always)]
    #[must_use]
    pub fn csof5(&mut self) -> CSOF5_W<5> {
        CSOF5_W::new(self)
    }
    #[doc = "Bit 6 - CSOF6"]
    #[inline(always)]
    #[must_use]
    pub fn csof6(&mut self) -> CSOF6_W<6> {
        CSOF6_W::new(self)
    }
    #[doc = "Bit 7 - CSOF7"]
    #[inline(always)]
    #[must_use]
    pub fn csof7(&mut self) -> CSOF7_W<7> {
        CSOF7_W::new(self)
    }
    #[doc = "Bit 8 - CSOF8"]
    #[inline(always)]
    #[must_use]
    pub fn csof8(&mut self) -> CSOF8_W<8> {
        CSOF8_W::new(self)
    }
    #[doc = "Bit 9 - CSOF9"]
    #[inline(always)]
    #[must_use]
    pub fn csof9(&mut self) -> CSOF9_W<9> {
        CSOF9_W::new(self)
    }
    #[doc = "Bit 10 - CSOF10"]
    #[inline(always)]
    #[must_use]
    pub fn csof10(&mut self) -> CSOF10_W<10> {
        CSOF10_W::new(self)
    }
    #[doc = "Bit 11 - CSOF11"]
    #[inline(always)]
    #[must_use]
    pub fn csof11(&mut self) -> CSOF11_W<11> {
        CSOF11_W::new(self)
    }
    #[doc = "Bit 12 - CSOF12"]
    #[inline(always)]
    #[must_use]
    pub fn csof12(&mut self) -> CSOF12_W<12> {
        CSOF12_W::new(self)
    }
    #[doc = "Bit 13 - CSOF13"]
    #[inline(always)]
    #[must_use]
    pub fn csof13(&mut self) -> CSOF13_W<13> {
        CSOF13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "request line multiplexer interrupt channel clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfr](index.html) module"]
pub struct CCFR_SPEC;
impl crate::RegisterSpec for CCFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ccfr::W](W) writer structure"]
impl crate::Writable for CCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFR to value 0"]
impl crate::Resettable for CCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
