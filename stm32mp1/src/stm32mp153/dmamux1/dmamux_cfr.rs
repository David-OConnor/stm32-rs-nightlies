#[doc = "Register `DMAMUX_CFR` writer"]
pub struct W(crate::W<DMAMUX_CFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAMUX_CFR_SPEC>;
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
impl From<crate::W<DMAMUX_CFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAMUX_CFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSOF0` writer - CSOF0"]
pub type CSOF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF1` writer - CSOF1"]
pub type CSOF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF2` writer - CSOF2"]
pub type CSOF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF3` writer - CSOF3"]
pub type CSOF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF4` writer - CSOF4"]
pub type CSOF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF5` writer - CSOF5"]
pub type CSOF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF6` writer - CSOF6"]
pub type CSOF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF7` writer - CSOF7"]
pub type CSOF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF8` writer - CSOF8"]
pub type CSOF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF9` writer - CSOF9"]
pub type CSOF9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF10` writer - CSOF10"]
pub type CSOF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF11` writer - CSOF11"]
pub type CSOF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF12` writer - CSOF12"]
pub type CSOF12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF13` writer - CSOF13"]
pub type CSOF13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF14` writer - CSOF14"]
pub type CSOF14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
#[doc = "Field `CSOF15` writer - CSOF15"]
pub type CSOF15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_CFR_SPEC, bool, O>;
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
    #[doc = "Bit 14 - CSOF14"]
    #[inline(always)]
    #[must_use]
    pub fn csof14(&mut self) -> CSOF14_W<14> {
        CSOF14_W::new(self)
    }
    #[doc = "Bit 15 - CSOF15"]
    #[inline(always)]
    #[must_use]
    pub fn csof15(&mut self) -> CSOF15_W<15> {
        CSOF15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAMUX request line multiplexer interrupt clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_cfr](index.html) module"]
pub struct DMAMUX_CFR_SPEC;
impl crate::RegisterSpec for DMAMUX_CFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dmamux_cfr::W](W) writer structure"]
impl crate::Writable for DMAMUX_CFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAMUX_CFR to value 0"]
impl crate::Resettable for DMAMUX_CFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
