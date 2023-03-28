#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUCKSEL` reader - WUCKSEL"]
pub type WUCKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WUCKSEL` writer - WUCKSEL"]
pub type WUCKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TSEDGE` reader - TSEDGE"]
pub type TSEDGE_R = crate::BitReader<bool>;
#[doc = "Field `TSEDGE` writer - TSEDGE"]
pub type TSEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `REFCKON` reader - REFCKON"]
pub type REFCKON_R = crate::BitReader<bool>;
#[doc = "Field `REFCKON` writer - REFCKON"]
pub type REFCKON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BYPSHAD` reader - BYPSHAD"]
pub type BYPSHAD_R = crate::BitReader<bool>;
#[doc = "Field `BYPSHAD` writer - BYPSHAD"]
pub type BYPSHAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FMT` reader - FMT"]
pub type FMT_R = crate::BitReader<bool>;
#[doc = "Field `FMT` writer - FMT"]
pub type FMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ALRAE` reader - ALRAE"]
pub type ALRAE_R = crate::BitReader<bool>;
#[doc = "Field `ALRAE` writer - ALRAE"]
pub type ALRAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ALRBE` reader - ALRBE"]
pub type ALRBE_R = crate::BitReader<bool>;
#[doc = "Field `ALRBE` writer - ALRBE"]
pub type ALRBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WUTE` reader - WUTE"]
pub type WUTE_R = crate::BitReader<bool>;
#[doc = "Field `WUTE` writer - WUTE"]
pub type WUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TSE` reader - TSE"]
pub type TSE_R = crate::BitReader<bool>;
#[doc = "Field `TSE` writer - TSE"]
pub type TSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ALRAIE` reader - ALRAIE"]
pub type ALRAIE_R = crate::BitReader<bool>;
#[doc = "Field `ALRAIE` writer - ALRAIE"]
pub type ALRAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ALRBIE` reader - ALRBIE"]
pub type ALRBIE_R = crate::BitReader<bool>;
#[doc = "Field `ALRBIE` writer - ALRBIE"]
pub type ALRBIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WUTIE` reader - WUTIE"]
pub type WUTIE_R = crate::BitReader<bool>;
#[doc = "Field `WUTIE` writer - WUTIE"]
pub type WUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TSIE` reader - TSIE"]
pub type TSIE_R = crate::BitReader<bool>;
#[doc = "Field `TSIE` writer - TSIE"]
pub type TSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ADD1H` reader - ADD1H"]
pub type ADD1H_R = crate::BitReader<bool>;
#[doc = "Field `ADD1H` writer - ADD1H"]
pub type ADD1H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SUB1H` reader - SUB1H"]
pub type SUB1H_R = crate::BitReader<bool>;
#[doc = "Field `SUB1H` writer - SUB1H"]
pub type SUB1H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BKP` reader - BKP"]
pub type BKP_R = crate::BitReader<bool>;
#[doc = "Field `BKP` writer - BKP"]
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `COSEL` reader - COSEL"]
pub type COSEL_R = crate::BitReader<bool>;
#[doc = "Field `COSEL` writer - COSEL"]
pub type COSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `POL` reader - POL"]
pub type POL_R = crate::BitReader<bool>;
#[doc = "Field `POL` writer - POL"]
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OSEL` reader - OSEL"]
pub type OSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSEL` writer - OSEL"]
pub type OSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COE` reader - COE"]
pub type COE_R = crate::BitReader<bool>;
#[doc = "Field `COE` writer - COE"]
pub type COE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ITSE` reader - ITSE"]
pub type ITSE_R = crate::BitReader<bool>;
#[doc = "Field `ITSE` writer - ITSE"]
pub type ITSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TAMPTS` reader - TAMPTS"]
pub type TAMPTS_R = crate::BitReader<bool>;
#[doc = "Field `TAMPTS` writer - TAMPTS"]
pub type TAMPTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TAMPOE` reader - TAMPOE"]
pub type TAMPOE_R = crate::BitReader<bool>;
#[doc = "Field `TAMPOE` writer - TAMPOE"]
pub type TAMPOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TAMPALRM_PU` reader - TAMPALRM_PU"]
pub type TAMPALRM_PU_R = crate::BitReader<bool>;
#[doc = "Field `TAMPALRM_PU` writer - TAMPALRM_PU"]
pub type TAMPALRM_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TAMPALRM_TYPE` reader - TAMPALRM_TYPE"]
pub type TAMPALRM_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `TAMPALRM_TYPE` writer - TAMPALRM_TYPE"]
pub type TAMPALRM_TYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OUT2EN` reader - OUT2EN"]
pub type OUT2EN_R = crate::BitReader<bool>;
#[doc = "Field `OUT2EN` writer - OUT2EN"]
pub type OUT2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - WUCKSEL"]
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - TSEDGE"]
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REFCKON"]
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BYPSHAD"]
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FMT"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - ALRAE"]
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ALRBE"]
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WUTE"]
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TSE"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ALRAIE"]
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ALRBIE"]
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - WUTIE"]
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TSIE"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ADD1H"]
    #[inline(always)]
    pub fn add1h(&self) -> ADD1H_R {
        ADD1H_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SUB1H"]
    #[inline(always)]
    pub fn sub1h(&self) -> SUB1H_R {
        SUB1H_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - COSEL"]
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - POL"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - OSEL"]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - COE"]
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ITSE"]
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TAMPTS"]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TAMPOE"]
    #[inline(always)]
    pub fn tampoe(&self) -> TAMPOE_R {
        TAMPOE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - TAMPALRM_PU"]
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TAMPALRM_PU_R {
        TAMPALRM_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TAMPALRM_TYPE"]
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TAMPALRM_TYPE_R {
        TAMPALRM_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OUT2EN"]
    #[inline(always)]
    pub fn out2en(&self) -> OUT2EN_R {
        OUT2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - WUCKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn wucksel(&mut self) -> WUCKSEL_W<0> {
        WUCKSEL_W::new(self)
    }
    #[doc = "Bit 3 - TSEDGE"]
    #[inline(always)]
    #[must_use]
    pub fn tsedge(&mut self) -> TSEDGE_W<3> {
        TSEDGE_W::new(self)
    }
    #[doc = "Bit 4 - REFCKON"]
    #[inline(always)]
    #[must_use]
    pub fn refckon(&mut self) -> REFCKON_W<4> {
        REFCKON_W::new(self)
    }
    #[doc = "Bit 5 - BYPSHAD"]
    #[inline(always)]
    #[must_use]
    pub fn bypshad(&mut self) -> BYPSHAD_W<5> {
        BYPSHAD_W::new(self)
    }
    #[doc = "Bit 6 - FMT"]
    #[inline(always)]
    #[must_use]
    pub fn fmt(&mut self) -> FMT_W<6> {
        FMT_W::new(self)
    }
    #[doc = "Bit 8 - ALRAE"]
    #[inline(always)]
    #[must_use]
    pub fn alrae(&mut self) -> ALRAE_W<8> {
        ALRAE_W::new(self)
    }
    #[doc = "Bit 9 - ALRBE"]
    #[inline(always)]
    #[must_use]
    pub fn alrbe(&mut self) -> ALRBE_W<9> {
        ALRBE_W::new(self)
    }
    #[doc = "Bit 10 - WUTE"]
    #[inline(always)]
    #[must_use]
    pub fn wute(&mut self) -> WUTE_W<10> {
        WUTE_W::new(self)
    }
    #[doc = "Bit 11 - TSE"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<11> {
        TSE_W::new(self)
    }
    #[doc = "Bit 12 - ALRAIE"]
    #[inline(always)]
    #[must_use]
    pub fn alraie(&mut self) -> ALRAIE_W<12> {
        ALRAIE_W::new(self)
    }
    #[doc = "Bit 13 - ALRBIE"]
    #[inline(always)]
    #[must_use]
    pub fn alrbie(&mut self) -> ALRBIE_W<13> {
        ALRBIE_W::new(self)
    }
    #[doc = "Bit 14 - WUTIE"]
    #[inline(always)]
    #[must_use]
    pub fn wutie(&mut self) -> WUTIE_W<14> {
        WUTIE_W::new(self)
    }
    #[doc = "Bit 15 - TSIE"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<15> {
        TSIE_W::new(self)
    }
    #[doc = "Bit 16 - ADD1H"]
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> ADD1H_W<16> {
        ADD1H_W::new(self)
    }
    #[doc = "Bit 17 - SUB1H"]
    #[inline(always)]
    #[must_use]
    pub fn sub1h(&mut self) -> SUB1H_W<17> {
        SUB1H_W::new(self)
    }
    #[doc = "Bit 18 - BKP"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<18> {
        BKP_W::new(self)
    }
    #[doc = "Bit 19 - COSEL"]
    #[inline(always)]
    #[must_use]
    pub fn cosel(&mut self) -> COSEL_W<19> {
        COSEL_W::new(self)
    }
    #[doc = "Bit 20 - POL"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<20> {
        POL_W::new(self)
    }
    #[doc = "Bits 21:22 - OSEL"]
    #[inline(always)]
    #[must_use]
    pub fn osel(&mut self) -> OSEL_W<21> {
        OSEL_W::new(self)
    }
    #[doc = "Bit 23 - COE"]
    #[inline(always)]
    #[must_use]
    pub fn coe(&mut self) -> COE_W<23> {
        COE_W::new(self)
    }
    #[doc = "Bit 24 - ITSE"]
    #[inline(always)]
    #[must_use]
    pub fn itse(&mut self) -> ITSE_W<24> {
        ITSE_W::new(self)
    }
    #[doc = "Bit 25 - TAMPTS"]
    #[inline(always)]
    #[must_use]
    pub fn tampts(&mut self) -> TAMPTS_W<25> {
        TAMPTS_W::new(self)
    }
    #[doc = "Bit 26 - TAMPOE"]
    #[inline(always)]
    #[must_use]
    pub fn tampoe(&mut self) -> TAMPOE_W<26> {
        TAMPOE_W::new(self)
    }
    #[doc = "Bit 29 - TAMPALRM_PU"]
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_pu(&mut self) -> TAMPALRM_PU_W<29> {
        TAMPALRM_PU_W::new(self)
    }
    #[doc = "Bit 30 - TAMPALRM_TYPE"]
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_type(&mut self) -> TAMPALRM_TYPE_W<30> {
        TAMPALRM_TYPE_W::new(self)
    }
    #[doc = "Bit 31 - OUT2EN"]
    #[inline(always)]
    #[must_use]
    pub fn out2en(&mut self) -> OUT2EN_W<31> {
        OUT2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
