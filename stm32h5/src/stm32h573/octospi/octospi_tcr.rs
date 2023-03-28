#[doc = "Register `OCTOSPI_TCR` reader"]
pub struct R(crate::R<OCTOSPI_TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTOSPI_TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTOSPI_TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTOSPI_TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTOSPI_TCR` writer"]
pub struct W(crate::W<OCTOSPI_TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTOSPI_TCR_SPEC>;
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
impl From<crate::W<OCTOSPI_TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTOSPI_TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCYC` reader - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated."]
pub type DCYC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCYC` writer - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated."]
pub type DCYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_TCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `DHQC` reader - Delay hold quarter cycle"]
pub type DHQC_R = crate::BitReader<bool>;
#[doc = "Field `DHQC` writer - Delay hold quarter cycle"]
pub type DHQC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_TCR_SPEC, bool, O>;
#[doc = "Field `SSHIFT` reader - Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)"]
pub type SSHIFT_R = crate::BitReader<bool>;
#[doc = "Field `SSHIFT` writer - Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)"]
pub type SSHIFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCTOSPI_TCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated."]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 28 - Delay hold quarter cycle"]
    #[inline(always)]
    pub fn dhqc(&self) -> DHQC_R {
        DHQC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)"]
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended to have at least six dummy cycles when using memories with DQS activated."]
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<0> {
        DCYC_W::new(self)
    }
    #[doc = "Bit 28 - Delay hold quarter cycle"]
    #[inline(always)]
    #[must_use]
    pub fn dhqc(&mut self) -> DHQC_W<28> {
        DHQC_W::new(self)
    }
    #[doc = "Bit 30 - Sample shift By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the external device. This bit allows the data to be sampled later in order to consider the external signal delays. The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode (when DDTR = 1.)"]
    #[inline(always)]
    #[must_use]
    pub fn sshift(&mut self) -> SSHIFT_W<30> {
        SSHIFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCTOSPI timing configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octospi_tcr](index.html) module"]
pub struct OCTOSPI_TCR_SPEC;
impl crate::RegisterSpec for OCTOSPI_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octospi_tcr::R](R) reader structure"]
impl crate::Readable for OCTOSPI_TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octospi_tcr::W](W) writer structure"]
impl crate::Writable for OCTOSPI_TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTOSPI_TCR to value 0"]
impl crate::Resettable for OCTOSPI_TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
