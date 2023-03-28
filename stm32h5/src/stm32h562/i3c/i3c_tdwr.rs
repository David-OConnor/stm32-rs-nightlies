#[doc = "Register `I3C_TDWR` writer"]
pub struct W(crate::W<I3C_TDWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I3C_TDWR_SPEC>;
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
impl From<crate::W<I3C_TDWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I3C_TDWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDB0` writer - 8-bit transmit data (earliest byte on I3C bus)"]
pub type TDB0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_TDWR_SPEC, u8, u8, 8, O>;
#[doc = "Field `TDB1` writer - 8-bit transmit data (next byte after TDB0\\[7:0\\]
on I3C bus)."]
pub type TDB1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_TDWR_SPEC, u8, u8, 8, O>;
#[doc = "Field `TDB2` writer - 8-bit transmit data (next byte after TDB1\\[7:0\\]
on I3C bus)."]
pub type TDB2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_TDWR_SPEC, u8, u8, 8, O>;
#[doc = "Field `TDB3` writer - 8-bit transmit data (latest byte on I3C bus)."]
pub type TDB3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_TDWR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - 8-bit transmit data (earliest byte on I3C bus)"]
    #[inline(always)]
    #[must_use]
    pub fn tdb0(&mut self) -> TDB0_W<0> {
        TDB0_W::new(self)
    }
    #[doc = "Bits 8:15 - 8-bit transmit data (next byte after TDB0\\[7:0\\]
on I3C bus)."]
    #[inline(always)]
    #[must_use]
    pub fn tdb1(&mut self) -> TDB1_W<8> {
        TDB1_W::new(self)
    }
    #[doc = "Bits 16:23 - 8-bit transmit data (next byte after TDB1\\[7:0\\]
on I3C bus)."]
    #[inline(always)]
    #[must_use]
    pub fn tdb2(&mut self) -> TDB2_W<16> {
        TDB2_W::new(self)
    }
    #[doc = "Bits 24:31 - 8-bit transmit data (latest byte on I3C bus)."]
    #[inline(always)]
    #[must_use]
    pub fn tdb3(&mut self) -> TDB3_W<24> {
        TDB3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I3C transmit data word register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i3c_tdwr](index.html) module"]
pub struct I3C_TDWR_SPEC;
impl crate::RegisterSpec for I3C_TDWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [i3c_tdwr::W](W) writer structure"]
impl crate::Writable for I3C_TDWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I3C_TDWR to value 0"]
impl crate::Resettable for I3C_TDWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
