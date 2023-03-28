#[doc = "Register `UCPDR` reader"]
pub struct R(crate::R<UCPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCPDR` writer"]
pub struct W(crate::W<UCPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCPDR_SPEC>;
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
impl From<crate::W<UCPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCPD_DBDIS` reader - USB Type-C and power delivery dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all case, either to stop this pull-down or to hand over control to the UCPD (which should therefore be initialized before doing the disable)."]
pub type UCPD_DBDIS_R = crate::BitReader<bool>;
#[doc = "Field `UCPD_DBDIS` writer - USB Type-C and power delivery dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all case, either to stop this pull-down or to hand over control to the UCPD (which should therefore be initialized before doing the disable)."]
pub type UCPD_DBDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UCPDR_SPEC, bool, O>;
#[doc = "Field `UCPD_STBY` reader - USB Type-c and Power delivery Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD, and it must be written to 0 after exiting the standby mode and before writing any UCPD register."]
pub type UCPD_STBY_R = crate::BitReader<bool>;
#[doc = "Field `UCPD_STBY` writer - USB Type-c and Power delivery Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD, and it must be written to 0 after exiting the standby mode and before writing any UCPD register."]
pub type UCPD_STBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, UCPDR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB Type-C and power delivery dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all case, either to stop this pull-down or to hand over control to the UCPD (which should therefore be initialized before doing the disable)."]
    #[inline(always)]
    pub fn ucpd_dbdis(&self) -> UCPD_DBDIS_R {
        UCPD_DBDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Type-c and Power delivery Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD, and it must be written to 0 after exiting the standby mode and before writing any UCPD register."]
    #[inline(always)]
    pub fn ucpd_stby(&self) -> UCPD_STBY_R {
        UCPD_STBY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Type-C and power delivery dead battery disable After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all case, either to stop this pull-down or to hand over control to the UCPD (which should therefore be initialized before doing the disable)."]
    #[inline(always)]
    #[must_use]
    pub fn ucpd_dbdis(&mut self) -> UCPD_DBDIS_W<0> {
        UCPD_DBDIS_W::new(self)
    }
    #[doc = "Bit 1 - USB Type-c and Power delivery Standby mode When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD, and it must be written to 0 after exiting the standby mode and before writing any UCPD register."]
    #[inline(always)]
    #[must_use]
    pub fn ucpd_stby(&mut self) -> UCPD_STBY_W<1> {
        UCPD_STBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR USB Type-C power delivery register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucpdr](index.html) module"]
pub struct UCPDR_SPEC;
impl crate::RegisterSpec for UCPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ucpdr::R](R) reader structure"]
impl crate::Readable for UCPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucpdr::W](W) writer structure"]
impl crate::Writable for UCPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCPDR to value 0"]
impl crate::Resettable for UCPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
