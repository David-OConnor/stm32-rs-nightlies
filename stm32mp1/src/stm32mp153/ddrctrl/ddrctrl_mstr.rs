#[doc = "Register `DDRCTRL_MSTR` reader"]
pub struct R(crate::R<DDRCTRL_MSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_MSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_MSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_MSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_MSTR` writer"]
pub struct W(crate::W<DDRCTRL_MSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_MSTR_SPEC>;
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
impl From<crate::W<DDRCTRL_MSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_MSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDR3` reader - DDR3"]
pub type DDR3_R = crate::BitReader<bool>;
#[doc = "Field `DDR3` writer - DDR3"]
pub type DDR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_MSTR_SPEC, bool, O>;
#[doc = "Field `LPDDR2` reader - LPDDR2"]
pub type LPDDR2_R = crate::BitReader<bool>;
#[doc = "Field `LPDDR2` writer - LPDDR2"]
pub type LPDDR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_MSTR_SPEC, bool, O>;
#[doc = "Field `LPDDR3` reader - LPDDR3"]
pub type LPDDR3_R = crate::BitReader<bool>;
#[doc = "Field `LPDDR3` writer - LPDDR3"]
pub type LPDDR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_MSTR_SPEC, bool, O>;
#[doc = "Field `BURSTCHOP` reader - BURSTCHOP"]
pub type BURSTCHOP_R = crate::BitReader<bool>;
#[doc = "Field `BURSTCHOP` writer - BURSTCHOP"]
pub type BURSTCHOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_MSTR_SPEC, bool, O>;
#[doc = "Field `EN_2T_TIMING_MODE` reader - EN_2T_TIMING_MODE"]
pub type EN_2T_TIMING_MODE_R = crate::BitReader<bool>;
#[doc = "Field `EN_2T_TIMING_MODE` writer - EN_2T_TIMING_MODE"]
pub type EN_2T_TIMING_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_MSTR_SPEC, bool, O>;
#[doc = "Field `DATA_BUS_WIDTH` reader - DATA_BUS_WIDTH"]
pub type DATA_BUS_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BUS_WIDTH` writer - DATA_BUS_WIDTH"]
pub type DATA_BUS_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_MSTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DLL_OFF_MODE` reader - DLL_OFF_MODE"]
pub type DLL_OFF_MODE_R = crate::BitReader<bool>;
#[doc = "Field `DLL_OFF_MODE` writer - DLL_OFF_MODE"]
pub type DLL_OFF_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_MSTR_SPEC, bool, O>;
#[doc = "Field `BURST_RDWR` reader - BURST_RDWR"]
pub type BURST_RDWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BURST_RDWR` writer - BURST_RDWR"]
pub type BURST_RDWR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_MSTR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - DDR3"]
    #[inline(always)]
    pub fn ddr3(&self) -> DDR3_R {
        DDR3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - LPDDR2"]
    #[inline(always)]
    pub fn lpddr2(&self) -> LPDDR2_R {
        LPDDR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPDDR3"]
    #[inline(always)]
    pub fn lpddr3(&self) -> LPDDR3_R {
        LPDDR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - BURSTCHOP"]
    #[inline(always)]
    pub fn burstchop(&self) -> BURSTCHOP_R {
        BURSTCHOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EN_2T_TIMING_MODE"]
    #[inline(always)]
    pub fn en_2t_timing_mode(&self) -> EN_2T_TIMING_MODE_R {
        EN_2T_TIMING_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - DATA_BUS_WIDTH"]
    #[inline(always)]
    pub fn data_bus_width(&self) -> DATA_BUS_WIDTH_R {
        DATA_BUS_WIDTH_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - DLL_OFF_MODE"]
    #[inline(always)]
    pub fn dll_off_mode(&self) -> DLL_OFF_MODE_R {
        DLL_OFF_MODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - BURST_RDWR"]
    #[inline(always)]
    pub fn burst_rdwr(&self) -> BURST_RDWR_R {
        BURST_RDWR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DDR3"]
    #[inline(always)]
    #[must_use]
    pub fn ddr3(&mut self) -> DDR3_W<0> {
        DDR3_W::new(self)
    }
    #[doc = "Bit 2 - LPDDR2"]
    #[inline(always)]
    #[must_use]
    pub fn lpddr2(&mut self) -> LPDDR2_W<2> {
        LPDDR2_W::new(self)
    }
    #[doc = "Bit 3 - LPDDR3"]
    #[inline(always)]
    #[must_use]
    pub fn lpddr3(&mut self) -> LPDDR3_W<3> {
        LPDDR3_W::new(self)
    }
    #[doc = "Bit 9 - BURSTCHOP"]
    #[inline(always)]
    #[must_use]
    pub fn burstchop(&mut self) -> BURSTCHOP_W<9> {
        BURSTCHOP_W::new(self)
    }
    #[doc = "Bit 10 - EN_2T_TIMING_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn en_2t_timing_mode(&mut self) -> EN_2T_TIMING_MODE_W<10> {
        EN_2T_TIMING_MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - DATA_BUS_WIDTH"]
    #[inline(always)]
    #[must_use]
    pub fn data_bus_width(&mut self) -> DATA_BUS_WIDTH_W<12> {
        DATA_BUS_WIDTH_W::new(self)
    }
    #[doc = "Bit 15 - DLL_OFF_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn dll_off_mode(&mut self) -> DLL_OFF_MODE_W<15> {
        DLL_OFF_MODE_W::new(self)
    }
    #[doc = "Bits 16:19 - BURST_RDWR"]
    #[inline(always)]
    #[must_use]
    pub fn burst_rdwr(&mut self) -> BURST_RDWR_W<16> {
        BURST_RDWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL master register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_mstr](index.html) module"]
pub struct DDRCTRL_MSTR_SPEC;
impl crate::RegisterSpec for DDRCTRL_MSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_mstr::R](R) reader structure"]
impl crate::Readable for DDRCTRL_MSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_mstr::W](W) writer structure"]
impl crate::Writable for DDRCTRL_MSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_MSTR to value 0x0004_0001"]
impl crate::Resettable for DDRCTRL_MSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0001;
}
