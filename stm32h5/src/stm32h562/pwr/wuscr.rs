#[doc = "Register `WUSCR` writer"]
pub struct W(crate::W<WUSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUSCR_SPEC>;
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
impl From<crate::W<WUSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWUF1` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUSCR_SPEC, bool, O>;
#[doc = "Field `CWUF2` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUSCR_SPEC, bool, O>;
#[doc = "Field `CWUF3` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUSCR_SPEC, bool, O>;
#[doc = "Field `CWUF4` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUSCR_SPEC, bool, O>;
#[doc = "Field `CWUF5` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUSCR_SPEC, bool, O>;
#[doc = "Field `CWUF6` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUSCR_SPEC, bool, O>;
#[doc = "Field `CWUF7` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUSCR_SPEC, bool, O>;
#[doc = "Field `CWUF8` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUSCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> CWUF1_W<0> {
        CWUF1_W::new(self)
    }
    #[doc = "Bit 1 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> CWUF2_W<1> {
        CWUF2_W::new(self)
    }
    #[doc = "Bit 2 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> CWUF3_W<2> {
        CWUF3_W::new(self)
    }
    #[doc = "Bit 3 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf4(&mut self) -> CWUF4_W<3> {
        CWUF4_W::new(self)
    }
    #[doc = "Bit 4 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf5(&mut self) -> CWUF5_W<4> {
        CWUF5_W::new(self)
    }
    #[doc = "Bit 5 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf6(&mut self) -> CWUF6_W<5> {
        CWUF6_W::new(self)
    }
    #[doc = "Bit 6 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf7(&mut self) -> CWUF7_W<6> {
        CWUF7_W::new(self)
    }
    #[doc = "Bit 7 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf8(&mut self) -> CWUF8_W<7> {
        CWUF8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR wakeup status clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wuscr](index.html) module"]
pub struct WUSCR_SPEC;
impl crate::RegisterSpec for WUSCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wuscr::W](W) writer structure"]
impl crate::Writable for WUSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUSCR to value 0"]
impl crate::Resettable for WUSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
