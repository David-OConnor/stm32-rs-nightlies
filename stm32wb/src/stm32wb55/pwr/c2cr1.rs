#[doc = "Register `C2CR1` reader"]
pub struct R(crate::R<C2CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2CR1` writer"]
pub struct W(crate::W<C2CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2CR1_SPEC>;
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
impl From<crate::W<C2CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPMS` reader - Low-power mode selection for CPU2"]
pub type LPMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPMS` writer - Low-power mode selection for CPU2"]
pub type LPMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `FPDR` reader - Flash power down mode during LPRun for CPU2"]
pub type FPDR_R = crate::BitReader<bool>;
#[doc = "Field `FPDR` writer - Flash power down mode during LPRun for CPU2"]
pub type FPDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR1_SPEC, bool, O>;
#[doc = "Field `FPDS` reader - Flash power down mode during LPSleep for CPU2"]
pub type FPDS_R = crate::BitReader<bool>;
#[doc = "Field `FPDS` writer - Flash power down mode during LPSleep for CPU2"]
pub type FPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR1_SPEC, bool, O>;
#[doc = "Field `BLEEWKUP` reader - BLE external wakeup signal"]
pub type BLEEWKUP_R = crate::BitReader<bool>;
#[doc = "Field `BLEEWKUP` writer - BLE external wakeup signal"]
pub type BLEEWKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR1_SPEC, bool, O>;
#[doc = "Field `_802EWKUP` reader - 802.15.4 external wakeup signal"]
pub type _802EWKUP_R = crate::BitReader<bool>;
#[doc = "Field `_802EWKUP` writer - 802.15.4 external wakeup signal"]
pub type _802EWKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Low-power mode selection for CPU2"]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Flash power down mode during LPRun for CPU2"]
    #[inline(always)]
    pub fn fpdr(&self) -> FPDR_R {
        FPDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash power down mode during LPSleep for CPU2"]
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 14 - BLE external wakeup signal"]
    #[inline(always)]
    pub fn bleewkup(&self) -> BLEEWKUP_R {
        BLEEWKUP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 802.15.4 external wakeup signal"]
    #[inline(always)]
    pub fn _802ewkup(&self) -> _802EWKUP_R {
        _802EWKUP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low-power mode selection for CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<0> {
        LPMS_W::new(self)
    }
    #[doc = "Bit 4 - Flash power down mode during LPRun for CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn fpdr(&mut self) -> FPDR_W<4> {
        FPDR_W::new(self)
    }
    #[doc = "Bit 5 - Flash power down mode during LPSleep for CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn fpds(&mut self) -> FPDS_W<5> {
        FPDS_W::new(self)
    }
    #[doc = "Bit 14 - BLE external wakeup signal"]
    #[inline(always)]
    #[must_use]
    pub fn bleewkup(&mut self) -> BLEEWKUP_W<14> {
        BLEEWKUP_W::new(self)
    }
    #[doc = "Bit 15 - 802.15.4 external wakeup signal"]
    #[inline(always)]
    #[must_use]
    pub fn _802ewkup(&mut self) -> _802EWKUP_W<15> {
        _802EWKUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 Power control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr1](index.html) module"]
pub struct C2CR1_SPEC;
impl crate::RegisterSpec for C2CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2cr1::R](R) reader structure"]
impl crate::Readable for C2CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2cr1::W](W) writer structure"]
impl crate::Writable for C2CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2CR1 to value 0"]
impl crate::Resettable for C2CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
