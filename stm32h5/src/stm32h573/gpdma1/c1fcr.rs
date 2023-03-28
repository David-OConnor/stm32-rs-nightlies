#[doc = "Register `C1FCR` writer"]
pub struct W(crate::W<C1FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1FCR_SPEC>;
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
impl From<crate::W<C1FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCF` writer - transfer complete flag clear"]
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1FCR_SPEC, bool, O>;
#[doc = "Field `HTF` writer - half transfer flag clear"]
pub type HTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1FCR_SPEC, bool, O>;
#[doc = "Field `DTEF` writer - data transfer error flag clear"]
pub type DTEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1FCR_SPEC, bool, O>;
#[doc = "Field `ULEF` writer - update link transfer error flag clear"]
pub type ULEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1FCR_SPEC, bool, O>;
#[doc = "Field `USEF` writer - user setting error flag clear"]
pub type USEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1FCR_SPEC, bool, O>;
#[doc = "Field `SUSPF` writer - completed suspension flag clear"]
pub type SUSPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1FCR_SPEC, bool, O>;
#[doc = "Field `TOF` writer - trigger overrun flag clear"]
pub type TOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1FCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 8 - transfer complete flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<8> {
        TCF_W::new(self)
    }
    #[doc = "Bit 9 - half transfer flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn htf(&mut self) -> HTF_W<9> {
        HTF_W::new(self)
    }
    #[doc = "Bit 10 - data transfer error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtef(&mut self) -> DTEF_W<10> {
        DTEF_W::new(self)
    }
    #[doc = "Bit 11 - update link transfer error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn ulef(&mut self) -> ULEF_W<11> {
        ULEF_W::new(self)
    }
    #[doc = "Bit 12 - user setting error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn usef(&mut self) -> USEF_W<12> {
        USEF_W::new(self)
    }
    #[doc = "Bit 13 - completed suspension flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn suspf(&mut self) -> SUSPF_W<13> {
        SUSPF_W::new(self)
    }
    #[doc = "Bit 14 - trigger overrun flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tof(&mut self) -> TOF_W<14> {
        TOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPDMA channel 1 flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1fcr](index.html) module"]
pub struct C1FCR_SPEC;
impl crate::RegisterSpec for C1FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [c1fcr::W](W) writer structure"]
impl crate::Writable for C1FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1FCR to value 0"]
impl crate::Resettable for C1FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
