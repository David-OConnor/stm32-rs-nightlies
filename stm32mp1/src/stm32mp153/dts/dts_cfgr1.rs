#[doc = "Register `DTS_CFGR1` reader"]
pub struct R(crate::R<DTS_CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTS_CFGR1` writer"]
pub struct W(crate::W<DTS_CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTS_CFGR1_SPEC>;
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
impl From<crate::W<DTS_CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTS_CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS1_EN` reader - TS1_EN"]
pub type TS1_EN_R = crate::BitReader<bool>;
#[doc = "Field `TS1_EN` writer - TS1_EN"]
pub type TS1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_CFGR1_SPEC, bool, O>;
#[doc = "Field `TS1_START` reader - TS1_START"]
pub type TS1_START_R = crate::BitReader<bool>;
#[doc = "Field `TS1_START` writer - TS1_START"]
pub type TS1_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_CFGR1_SPEC, bool, O>;
#[doc = "Field `TS1_INTRIG_SEL` reader - TS1_INTRIG_SEL"]
pub type TS1_INTRIG_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS1_INTRIG_SEL` writer - TS1_INTRIG_SEL"]
pub type TS1_INTRIG_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DTS_CFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `TS1_SMP_TIME` reader - TS1_SMP_TIME"]
pub type TS1_SMP_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS1_SMP_TIME` writer - TS1_SMP_TIME"]
pub type TS1_SMP_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DTS_CFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `REFCLK_SEL` reader - REFCLK_SEL"]
pub type REFCLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `REFCLK_SEL` writer - REFCLK_SEL"]
pub type REFCLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_CFGR1_SPEC, bool, O>;
#[doc = "Field `Q_MEAS_opt` reader - Q_MEAS_opt"]
pub type Q_MEAS_OPT_R = crate::BitReader<bool>;
#[doc = "Field `Q_MEAS_opt` writer - Q_MEAS_opt"]
pub type Q_MEAS_OPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_CFGR1_SPEC, bool, O>;
#[doc = "Field `HSREF_CLK_DIV` reader - HSREF_CLK_DIV"]
pub type HSREF_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSREF_CLK_DIV` writer - HSREF_CLK_DIV"]
pub type HSREF_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DTS_CFGR1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - TS1_EN"]
    #[inline(always)]
    pub fn ts1_en(&self) -> TS1_EN_R {
        TS1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - TS1_START"]
    #[inline(always)]
    pub fn ts1_start(&self) -> TS1_START_R {
        TS1_START_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - TS1_INTRIG_SEL"]
    #[inline(always)]
    pub fn ts1_intrig_sel(&self) -> TS1_INTRIG_SEL_R {
        TS1_INTRIG_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - TS1_SMP_TIME"]
    #[inline(always)]
    pub fn ts1_smp_time(&self) -> TS1_SMP_TIME_R {
        TS1_SMP_TIME_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - REFCLK_SEL"]
    #[inline(always)]
    pub fn refclk_sel(&self) -> REFCLK_SEL_R {
        REFCLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Q_MEAS_opt"]
    #[inline(always)]
    pub fn q_meas_opt(&self) -> Q_MEAS_OPT_R {
        Q_MEAS_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:30 - HSREF_CLK_DIV"]
    #[inline(always)]
    pub fn hsref_clk_div(&self) -> HSREF_CLK_DIV_R {
        HSREF_CLK_DIV_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TS1_EN"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_en(&mut self) -> TS1_EN_W<0> {
        TS1_EN_W::new(self)
    }
    #[doc = "Bit 4 - TS1_START"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_start(&mut self) -> TS1_START_W<4> {
        TS1_START_W::new(self)
    }
    #[doc = "Bits 8:11 - TS1_INTRIG_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_intrig_sel(&mut self) -> TS1_INTRIG_SEL_W<8> {
        TS1_INTRIG_SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - TS1_SMP_TIME"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_smp_time(&mut self) -> TS1_SMP_TIME_W<16> {
        TS1_SMP_TIME_W::new(self)
    }
    #[doc = "Bit 20 - REFCLK_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn refclk_sel(&mut self) -> REFCLK_SEL_W<20> {
        REFCLK_SEL_W::new(self)
    }
    #[doc = "Bit 21 - Q_MEAS_opt"]
    #[inline(always)]
    #[must_use]
    pub fn q_meas_opt(&mut self) -> Q_MEAS_OPT_W<21> {
        Q_MEAS_OPT_W::new(self)
    }
    #[doc = "Bits 24:30 - HSREF_CLK_DIV"]
    #[inline(always)]
    #[must_use]
    pub fn hsref_clk_div(&mut self) -> HSREF_CLK_DIV_W<24> {
        HSREF_CLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTS_CFGR1 is the configuration register for temperature sensor 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_cfgr1](index.html) module"]
pub struct DTS_CFGR1_SPEC;
impl crate::RegisterSpec for DTS_CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_cfgr1::R](R) reader structure"]
impl crate::Readable for DTS_CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dts_cfgr1::W](W) writer structure"]
impl crate::Writable for DTS_CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTS_CFGR1 to value 0"]
impl crate::Resettable for DTS_CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
