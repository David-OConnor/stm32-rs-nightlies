#[doc = "Register `SYSCFG_PMCSETR` reader"]
pub struct R(crate::R<SYSCFG_PMCSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_PMCSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_PMCSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_PMCSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_PMCSETR` writer"]
pub struct W(crate::W<SYSCFG_PMCSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_PMCSETR_SPEC>;
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
impl From<crate::W<SYSCFG_PMCSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_PMCSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C1_FMP` reader - I2C1_FMP"]
pub type I2C1_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C1_FMP` writer - I2C1_FMP"]
pub type I2C1_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_PMCSETR_SPEC, bool, O>;
#[doc = "Field `I2C2_FMP` reader - I2C2_FMP"]
pub type I2C2_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C2_FMP` writer - I2C2_FMP"]
pub type I2C2_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_PMCSETR_SPEC, bool, O>;
#[doc = "Field `I2C3_FMP` reader - I2C3_FMP"]
pub type I2C3_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C3_FMP` writer - I2C3_FMP"]
pub type I2C3_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_PMCSETR_SPEC, bool, O>;
#[doc = "Field `I2C4_FMP` reader - I2C4_FMP"]
pub type I2C4_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C4_FMP` writer - I2C4_FMP"]
pub type I2C4_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_PMCSETR_SPEC, bool, O>;
#[doc = "Field `I2C5_FMP` reader - I2C5_FMP"]
pub type I2C5_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C5_FMP` writer - I2C5_FMP"]
pub type I2C5_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_PMCSETR_SPEC, bool, O>;
#[doc = "Field `I2C6_FMP` reader - I2C6_FMP"]
pub type I2C6_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C6_FMP` writer - I2C6_FMP"]
pub type I2C6_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_PMCSETR_SPEC, bool, O>;
#[doc = "Field `EN_BOOSTER` reader - EN_BOOSTER"]
pub type EN_BOOSTER_R = crate::BitReader<bool>;
#[doc = "Field `EN_BOOSTER` writer - EN_BOOSTER"]
pub type EN_BOOSTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_PMCSETR_SPEC, bool, O>;
#[doc = "Field `ANASWVDD` reader - ANASWVDD"]
pub type ANASWVDD_R = crate::BitReader<bool>;
#[doc = "Field `ANASWVDD` writer - ANASWVDD"]
pub type ANASWVDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_PMCSETR_SPEC, bool, O>;
#[doc = "Field `ETH_CLK_SEL` reader - ETH_CLK_SEL"]
pub type ETH_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `ETH_CLK_SEL` writer - ETH_CLK_SEL"]
pub type ETH_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_PMCSETR_SPEC, bool, O>;
#[doc = "Field `ETH_REF_CLK_SEL` reader - ETH_REF_CLK_SEL"]
pub type ETH_REF_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `ETH_REF_CLK_SEL` writer - ETH_REF_CLK_SEL"]
pub type ETH_REF_CLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYSCFG_PMCSETR_SPEC, bool, O>;
#[doc = "Field `ETH_SELMII` reader - ETH_SELMII"]
pub type ETH_SELMII_R = crate::BitReader<bool>;
#[doc = "Field `ETH_SELMII` writer - ETH_SELMII"]
pub type ETH_SELMII_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_PMCSETR_SPEC, bool, O>;
#[doc = "Field `ETH_SEL` reader - ETH_SEL"]
pub type ETH_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETH_SEL` writer - ETH_SEL"]
pub type ETH_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSCFG_PMCSETR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ANA0_SEL` reader - ANA0_SEL"]
pub type ANA0_SEL_R = crate::BitReader<bool>;
#[doc = "Field `ANA0_SEL` writer - ANA0_SEL"]
pub type ANA0_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_PMCSETR_SPEC, bool, O>;
#[doc = "Field `ANA1_SEL` reader - ANA1_SEL"]
pub type ANA1_SEL_R = crate::BitReader<bool>;
#[doc = "Field `ANA1_SEL` writer - ANA1_SEL"]
pub type ANA1_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_PMCSETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I2C1_FMP"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C2_FMP"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C3_FMP"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C4_FMP"]
    #[inline(always)]
    pub fn i2c4_fmp(&self) -> I2C4_FMP_R {
        I2C4_FMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C5_FMP"]
    #[inline(always)]
    pub fn i2c5_fmp(&self) -> I2C5_FMP_R {
        I2C5_FMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C6_FMP"]
    #[inline(always)]
    pub fn i2c6_fmp(&self) -> I2C6_FMP_R {
        I2C6_FMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - EN_BOOSTER"]
    #[inline(always)]
    pub fn en_booster(&self) -> EN_BOOSTER_R {
        EN_BOOSTER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ANASWVDD"]
    #[inline(always)]
    pub fn anaswvdd(&self) -> ANASWVDD_R {
        ANASWVDD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - ETH_CLK_SEL"]
    #[inline(always)]
    pub fn eth_clk_sel(&self) -> ETH_CLK_SEL_R {
        ETH_CLK_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ETH_REF_CLK_SEL"]
    #[inline(always)]
    pub fn eth_ref_clk_sel(&self) -> ETH_REF_CLK_SEL_R {
        ETH_REF_CLK_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - ETH_SELMII"]
    #[inline(always)]
    pub fn eth_selmii(&self) -> ETH_SELMII_R {
        ETH_SELMII_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - ETH_SEL"]
    #[inline(always)]
    pub fn eth_sel(&self) -> ETH_SEL_R {
        ETH_SEL_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - ANA0_SEL"]
    #[inline(always)]
    pub fn ana0_sel(&self) -> ANA0_SEL_R {
        ANA0_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ANA1_SEL"]
    #[inline(always)]
    pub fn ana1_sel(&self) -> ANA1_SEL_R {
        ANA1_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<0> {
        I2C1_FMP_W::new(self)
    }
    #[doc = "Bit 1 - I2C2_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<1> {
        I2C2_FMP_W::new(self)
    }
    #[doc = "Bit 2 - I2C3_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<2> {
        I2C3_FMP_W::new(self)
    }
    #[doc = "Bit 3 - I2C4_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4_fmp(&mut self) -> I2C4_FMP_W<3> {
        I2C4_FMP_W::new(self)
    }
    #[doc = "Bit 4 - I2C5_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c5_fmp(&mut self) -> I2C5_FMP_W<4> {
        I2C5_FMP_W::new(self)
    }
    #[doc = "Bit 5 - I2C6_FMP"]
    #[inline(always)]
    #[must_use]
    pub fn i2c6_fmp(&mut self) -> I2C6_FMP_W<5> {
        I2C6_FMP_W::new(self)
    }
    #[doc = "Bit 8 - EN_BOOSTER"]
    #[inline(always)]
    #[must_use]
    pub fn en_booster(&mut self) -> EN_BOOSTER_W<8> {
        EN_BOOSTER_W::new(self)
    }
    #[doc = "Bit 9 - ANASWVDD"]
    #[inline(always)]
    #[must_use]
    pub fn anaswvdd(&mut self) -> ANASWVDD_W<9> {
        ANASWVDD_W::new(self)
    }
    #[doc = "Bit 16 - ETH_CLK_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn eth_clk_sel(&mut self) -> ETH_CLK_SEL_W<16> {
        ETH_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 17 - ETH_REF_CLK_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn eth_ref_clk_sel(&mut self) -> ETH_REF_CLK_SEL_W<17> {
        ETH_REF_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 20 - ETH_SELMII"]
    #[inline(always)]
    #[must_use]
    pub fn eth_selmii(&mut self) -> ETH_SELMII_W<20> {
        ETH_SELMII_W::new(self)
    }
    #[doc = "Bits 21:23 - ETH_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn eth_sel(&mut self) -> ETH_SEL_W<21> {
        ETH_SEL_W::new(self)
    }
    #[doc = "Bit 24 - ANA0_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ana0_sel(&mut self) -> ANA0_SEL_W<24> {
        ANA0_SEL_W::new(self)
    }
    #[doc = "Bit 25 - ANA1_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ana1_sel(&mut self) -> ANA1_SEL_W<25> {
        ANA1_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG peripheral mode configuration set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_pmcsetr](index.html) module"]
pub struct SYSCFG_PMCSETR_SPEC;
impl crate::RegisterSpec for SYSCFG_PMCSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_pmcsetr::R](R) reader structure"]
impl crate::Readable for SYSCFG_PMCSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_pmcsetr::W](W) writer structure"]
impl crate::Writable for SYSCFG_PMCSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG_PMCSETR to value 0"]
impl crate::Resettable for SYSCFG_PMCSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
